mod crud;

#[tokio::main]
async fn main() {
    //let conn = crud::get_connection().await;
    //crud::setup(&conn).await.unwrap();

    // let ingredients = crud::Ingredient::read(&conn).await.unwrap();
    // dbg!(ingredients)
    //
    // let ingredient_id = 2;
    // let ingredient = crud::Ingredient::read_one(&conn, ingredient_id).await.unwrap();
    // dbg!(ingredient);
    //
    //let tags = crud::Tag::read(&conn).await.unwrap();
    //dbg!(tags);
    //
    //let tag_id = 2;
    //let tag = crud::Tag::read_one(&conn, tag_id).await.unwrap();
    //dbg!(tag);
}
