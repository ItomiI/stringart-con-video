import cv2
import os
import sys

if __name__ == '__main__':
    carpetaimgs = sys.argv[1]
    nombre = sys.argv[2]
    ext = sys.argv[3]
    n = sys.argv[4]
    #dir_path = 'C:/Users/tomas/Desktop/stringart/stringvideo/hola3/'
    dir_path = carpetaimgs
    output = nombre+".mp4"

    images = []

    for i in range(1, int(n)):
        images.append('imagen'+str(i)+'.'+ ext)

    # Determine the width and height from the first image
    image_path = os.path.join(dir_path, images[0])
    frame = cv2.imread(image_path)
    cv2.imshow('video',frame)
    height, width, channels = frame.shape

    # Define the codec and create VideoWriter object
    fourcc = cv2.VideoWriter_fourcc(*'mp4v') # Be sure to use lower case
    out = cv2.VideoWriter(output, fourcc, 100.0, (width, height))

    for image in images:

        image_path = os.path.join(dir_path, image)
        frame = cv2.imread(image_path)

        out.write(frame) # Write out frame to video

        cv2.imshow('video',frame)
        if (cv2.waitKey(1) & 0xFF) == ord('q'): # Hit `q` to exit
            break

    # Release everything if job is finished
    out.release()
    cv2.destroyAllWindows()
