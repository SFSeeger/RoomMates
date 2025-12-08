#[cfg(feature = "server")]
mod test_db_mod {
    // .....

    use chrono::{NaiveDate, NaiveTime};
    use entity::{
        event, group, is_in_group, links::FriendEvents, shared_friend_event, shared_group_event,
        user,
    };
    use sea_orm::{
        ColumnTrait, Database, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter, Set,
    };

    #[cfg(feature = "server")]
    #[tokio::test]
    pub async fn db_model_test() -> Result<(), DbErr> {
        // Connecting SQLite
        let db = Database::connect("sqlite::memory:").await?;

        build_db(&db).await?;

        insert(&db).await?;

        assert_eq!(check_group_membership(&db).await?, ());
        assert_eq!(check_shared_friend_events(&db).await?, ());
        assert_eq!(check_shared_group_events(&db).await?, ());

        Ok(())
    }

    pub async fn build_db(db: &DatabaseConnection) -> Result<(), DbErr> {
        db.get_schema_builder()
            .register(user::Entity)
            .register(event::Entity)
            .register(group::Entity)
            .register(shared_friend_event::Entity)
            .register(shared_group_event::Entity)
            .register(is_in_group::Entity)
            .apply(db)
            .await?;

        Ok(())
    }

    pub async fn insert(db: &DatabaseConnection) -> Result<(), DbErr> {
        let user1: user::ActiveModel = user::ActiveModel {
            id: Set(1),
            email: Set("proton".to_owned()),
            first_name: Set("kara".to_owned()),
            last_name: Set("rau".to_owned()),
            password: Set("pass".to_owned()),
        };

        user::Entity::insert(user1).exec(db).await?;

        let user2: user::ActiveModel = user::ActiveModel {
            id: Set(5),
            email: Set("gmail".to_owned()),
            first_name: Set("simon".to_owned()),
            last_name: Set("hhh".to_owned()),
            password: Set("word".to_owned()),
        };

        user::Entity::insert(user2).exec(db).await?;

        let group1: group::ActiveModel = group::ActiveModel {
            id: Set(8),
            name: Set("group1".to_owned()),
        };

        group::Entity::insert(group1).exec(db).await?;

        let ev1: event::ActiveModel = event::ActiveModel {
            id: Set(1),
            title: Set("ev1".to_owned()),
            reocurring: Set(false),
            is_private: Set(false),
            //desc: Set("nya".to_owned()),
            //location: Set("owo".to_owned()),
            date: Set(NaiveDate::from_ymd_opt(2026, 1, 8).unwrap()),
            start_time: Set(NaiveTime::from_hms_milli_opt(8, 59, 59, 1_000).unwrap()),
            end_time: Set(NaiveTime::from_hms_milli_opt(8, 59, 59, 1_000).unwrap()),
            weekday: Set(event::Weekday::Monday),
            owner_id: Set(1),
            ..Default::default()
        };

        event::Entity::insert(ev1).exec(db).await?;

        let ev2: event::ActiveModel = event::ActiveModel {
            id: Set(2),
            title: Set("ev2".to_owned()),
            reocurring: Set(true),
            is_private: Set(true),
            desc: Set(Some("nya".to_owned())),
            location: Set(Some("owo".to_owned())),
            date: Set(NaiveDate::from_ymd_opt(2026, 1, 8).unwrap()),
            start_time: Set(NaiveTime::from_hms_milli_opt(8, 59, 59, 1_000).unwrap()),
            end_time: Set(NaiveTime::from_hms_milli_opt(8, 59, 59, 1_000).unwrap()),
            weekday: Set(event::Weekday::Wednesday),
            owner_id: Set(1),
        };

        event::Entity::insert(ev2).exec(db).await?;

        Ok(())
    }

    pub async fn check_group_membership(db: &DatabaseConnection) -> Result<(), DbErr> {
        let res1: Vec<is_in_group::Model> = is_in_group::Entity::find()
            .filter(is_in_group::Column::UserId.eq(1))
            .all(db)
            .await?;

        assert!(res1.is_empty());

        let link: is_in_group::ActiveModel = is_in_group::ActiveModel {
            user_id: Set(1),
            group_id: Set(8),
        };

        is_in_group::Entity::insert(link).exec(db).await?;

        let res2: Vec<is_in_group::Model> = is_in_group::Entity::find()
            .filter(is_in_group::Column::UserId.eq(1))
            .all(db)
            .await?;

        assert_eq!(res2[0].group_id, 8);

        let link2: is_in_group::ActiveModel = is_in_group::ActiveModel {
            user_id: Set(5),
            group_id: Set(8),
        };

        is_in_group::Entity::insert(link2).exec(db).await?;

        Ok(())
    }

    pub async fn check_shared_friend_events(db: &DatabaseConnection) -> Result<(), DbErr> {
        let link: shared_friend_event::ActiveModel = shared_friend_event::ActiveModel {
            user_id: Set(5),
            event_id: Set(2),
        };

        shared_friend_event::Entity::insert(link).exec(db).await?;

        let u2: user::Model = Option::unwrap(user::Entity::find_by_id(5).one(db).await?);

        let res1: Vec<event::Model> = u2.find_linked(FriendEvents).all(db).await?;

        assert_eq!(res1[0].id, 2);

        let link2: shared_friend_event::ActiveModel = shared_friend_event::ActiveModel {
            user_id: Set(5),
            event_id: Set(1),
        };

        shared_friend_event::Entity::insert(link2).exec(db).await?;

        let res2: Vec<event::Model> = u2.find_linked(FriendEvents).all(db).await?;

        assert_eq!(res2.len(), 2);

        //let u1: user::Model = Option::unwrap(user::Entity::find_by_id(1).one(db).await?);

        //let res3 = event::Entity::find().filter(event::Column::OwnerId.eq(1)).f

        Ok(())
    }

    pub async fn check_shared_group_events(db: &DatabaseConnection) -> Result<(), DbErr> {
        let link: shared_group_event::ActiveModel = shared_group_event::ActiveModel {
            group_id: Set(8),
            event_id: Set(1),
        };

        shared_group_event::Entity::insert(link).exec(db).await?;

        let e1: event::Model = Option::unwrap(event::Entity::find_by_id(1).one(db).await?);

        let res1: Vec<group::Model> = e1.find_related(group::Entity).all(db).await?;

        assert_eq!(res1[0].id, 8);

        let g1: group::Model = Option::unwrap(group::Entity::find_by_id(8).one(db).await?);

        let res2: Vec<event::Model> = g1.find_related(event::Entity).all(db).await?;

        assert_eq!(res2[0].id, 1);

        Ok(())
    }
}
