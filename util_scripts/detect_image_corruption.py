import os
import PIL
from PIL import Image

path = "data"
for i in os.listdir(path):
	file_path = os.path.join(path, i)
	if os.path.isfile(file_path):
		try:
			with Image.open(file_path) as im:
				im_data = im.verify()
			with Image.open(file_path) as im:
				file_name, file_extension = os.path.splitext(file_path)
				im = im.convert('RGB')
				if im.format != file_extension[1:].upper():
					# os.remove(file_path)
					im.save(f"{file_name}.jpeg", format="JPEG")
					os.remove(file_path)
		except PIL.UnidentifiedImageError:
			print(f"Error: {i}")
