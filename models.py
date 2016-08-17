from sqlalchemy import Column, ForeignKey, Integer, String, Float, DateTime
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import relationship
from sqlalchemy import create_engine

Base = declarative_base()


class ChangeHeader(Base):
    __tablename__ = 'change_header'
    # Here we define columns for the table person
    # Notice that each column is also a normal Python instance attribute.
    id = Column(Integer, primary_key=True)
    time = Column(DateTime)
    table_id = Column(String(50), ForeignKey('table.name'))
    table = relationship(Table, back_populates="headers")


class ChangeInfo(Base):
    __tablename__ = "change_info"
    id = Column(Integer, primary_key=True)
    x = Column(Integer)
    y = Column(Integer)
    rot = Column(Integer)
    type = Column(Integer)
    header_id = Column(Integer, ForeignKey('change_header.id'))
    header = relationship(ChangeHeader, back_populates="changes")


class Table(Base):
    __tablename__ = 'table'
    # Here we define columns for the table address.
    # Notice that each column is also a normal Python instance attribute.
    # id = Column(Integer, primary_key=True)
    name = Column(String(50), primary_key=True)

    # version = Column(String(50))


class Comment(Base):
    __tablename__ = "comment"
    id = Column(Integer, primary_key=True)
    text = Column(String(50))
    latitude = Column(Float)
    longitude = Column(Float)


class CommentBuilding(Base):
    __tablename__ = "comment_building"
    id = Column(Integer, primary_key=True)
    x = Column(Integer)
    y = Column(Integer)
    comment_id = Column(Integer, ForeignKey("comment.id"))
    comment = relationship(Comment, back_populates="comment")



# Create an engine that stores data in the local directory's
# sqlalchemy_example.db file.
engine = create_engine('sqlite:///tables.db')

# Create all tables in the engine. This is equivalent to "Create Table"
# statements in raw SQL.
Base.metadata.create_all(engine)
