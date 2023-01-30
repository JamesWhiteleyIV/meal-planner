from sqlalchemy.orm import Session

import models, schemas


'''
def get_tags(db: Session, tag_id: int):
    return db.query(models.Tag).filter(models.Tag.id == tag_id).first()
'''

def get_tags(db: Session, skip: int = 0, limit: int = 100):
    return db.query(models.Tag).offset(skip).limit(limit).all()


def create_tag(db: Session, tag: schemas.TagCreate):
    db_tag = models.Tag(name=tag.name)
    db.add(db_tag)
    db.commit()
    db.refresh(db_tag)
    return db_tag


'''
def get_user_by_email(db: Session, email: str):
    return db.query(models.User).filter(models.User.email == email).first()


def get_users(db: Session, skip: int = 0, limit: int = 100):
    return db.query(models.User).offset(skip).limit(limit).all()



def get_items(db: Session, skip: int = 0, limit: int = 100):
    return db.query(models.Item).offset(skip).limit(limit).all()


def create_user_item(db: Session, item: schemas.ItemCreate, user_id: int):
    db_item = models.Item(**item.dict(), owner_id=user_id)
    db.add(db_item)
    db.commit()
    db.refresh(db_item)
    return db_item
'''


if __name__ == "__main__":
    from database import SessionLocal, engine

    models.Base.metadata.create_all(bind=engine)

    try:
        db = SessionLocal()
        tag = schemas.TagCreate(name="potty")
        print(create_tag(db, tag).__dict__)
        for tag in get_tags(db):
            print(tag.__dict__)
    finally:
        db.close()
