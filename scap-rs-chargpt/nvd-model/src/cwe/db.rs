use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel::{
  ExpressionMethods, Insertable, PgConnection, QueryDsl, RunQueryDsl, TextExpressionMethods,
};

use crate::cwe::{Cwe, QueryCwe};
use crate::error::{DBError, DBResult};
use crate::pagination::ListResponse;
use crate::schema::cwes;
use crate::{Connection, DB};

#[derive(Insertable)]
#[diesel(table_name = cwes)]
pub struct CreateCwe {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub status: String,
}

#[derive(Insertable)]
#[diesel(table_name = cwes)]
pub struct UpdateCwe {
  pub id: i32,
  pub name_zh: String,
  pub description_zh: String,
  pub remediation: String,
}

impl QueryCwe {
  fn query<'a>(
    &'a self,
    _conn: &mut PgConnection,
    mut query: cwes::BoxedQuery<'a, DB>,
  ) -> DBResult<cwes::BoxedQuery<'a, DB>> {
    if let Some(name) = &self.name {
      let name = format!("%{name}%");
      query = query.filter(cwes::name.like(name));
    }
    if let Some(id) = &self.id {
      query = query.filter(cwes::id.eq(id));
    }
    Ok(query)
  }
  fn total(&self, conn: &mut PgConnection) -> DBResult<i64> {
    let query = self.query(conn, cwes::table.into_boxed())?;
    // 统计查询全部，分页用
    Ok(
      query
        .select(diesel::dsl::count(cwes::id))
        .first::<i64>(conn)?,
    )
  }
}

impl Cwe {
  // 创建弱点枚举
  pub fn create(conn: &mut Connection, args: &CreateCwe) -> DBResult<Self> {
    if let Err(err) = diesel::insert_into(cwes::table).values(args).execute(conn) {
      // 重复了，说明已经存在弱点
      match err {
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {}
        _ => {
          return Err(DBError::DieselError { source: err });
        }
      }
    }
    Ok(
      // mysql 不支持 get_result，要再查一次得到插入结果
      cwes::dsl::cwes
        .filter(cwes::name.eq(&args.name))
        .first::<Cwe>(conn)?,
    )
  }
  pub fn query_by_id(conn: &mut PgConnection, id: &i32) -> DBResult<Self> {
    Ok(
      cwes::dsl::cwes
        .filter(cwes::id.eq(id))
        .first::<Self>(conn)?,
    )
  }
  pub fn update(conn: &mut PgConnection, args: &UpdateCwe) -> DBResult<Self> {
    // 更新这个KB
    let _id = diesel::update(cwes::table.filter(cwes::id.eq(&args.id)))
      .set((
        cwes::name_zh.eq(&args.name_zh),
        cwes::description_zh.eq(&args.description_zh),
        cwes::remediation.eq(&args.remediation),
      ))
      .execute(conn);
    // mysql 不支持 get_result，要再查一次得到插入结果
    Self::query_by_id(conn, &args.id)
  }
  pub fn query(
    conn: &mut PgConnection,
    args: &QueryCwe,
  ) -> DBResult<ListResponse<Cwe, QueryCwe>> {
    let total = args.total(conn)?;
    let page = args.page.unwrap_or(0).abs();
    let size = std::cmp::min(args.size.to_owned().unwrap_or(10).abs(), 10);
    let result = {
      let query = args.query(conn, cwes::table.into_boxed())?;
      query
        .offset(page)
        .limit(size)
        .order(cwes::name.asc())
        .load::<Cwe>(conn)?
    };
    Ok(ListResponse::new(result, total, page, size, args.clone()))
  }
}
