import argparse
import psycopg2


def insert_sql(params, sql_query, values):
    conn = None
    try:
        conn = psycopg2.connect(**params)
        cur = conn.cursor()
        cur.executemany(sql_query, values)
        conn.commit()
        cur.close()
    except (Exception, psycopg2.DatabaseError) as error:
        raise Exception(error)
    finally:
        if conn is not None:
            conn.close()


def insert_tag_format_list(params, table_name, values):
    insert_sql(params, f"INSERT INTO {table_name} (name) VALUES (%s);", values)


def insert_media(params, values):
    insert_sql(params, f"INSERT INTO media (url, format_id, tag_id) VALUES (%s, %s, %s);", values)


def populate_empty_db():
    params = {"host": "localhost", "database": "cudi_db", "user": "gmx", "password": "1234"}
    insert_tag_format_list(params, "format", [["PNG"], ["JPEG"], ["GIF"], ["BMP"], ["WEBP"]])
    insert_tag_format_list(params, "tag", [["TEST"]])
    insert_media(params, [["data/init/loading.jpeg", 2, 1]])


def run():
    parser = argparse.ArgumentParser(prog="Database Handler", description="Automate the filling of the database")
    group = parser.add_mutually_exclusive_group()
    group.add_argument("-fs", "--from-scratch", action="store_const", dest="f", const=populate_empty_db)

    args = parser.parse_args()
    args.f()


if __name__ == "__main__":
    run()
