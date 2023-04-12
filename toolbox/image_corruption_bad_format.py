import os
import PIL
from PIL import Image


def check_files(file_path: str):
    try:
        with Image.open(file_path) as im:
            im_data = im.verify()
            if im_data is not None:
                # corrupted image
                os.remove(file_path)
        with Image.open(file_path) as im:
            file_name, file_extension = os.path.splitext(file_path)
            if im.format != file_extension[1:].upper():
                im = im.convert("RGB")
                im.save(f"{file_name}.jpeg", format="JPEG")
                os.remove(file_path)
    except PIL.UnidentifiedImageError:
        print(f"Error: {file_path}")


def check_folder(path: str):
    for i in os.listdir(path):
        full_path = os.path.join(path, i)
        if os.path.isfile(full_path):
            check_files(full_path)
        else:
            check_folder(full_path)


def run():
    check_folder("data")


if __name__ == "__main__":
    run()
