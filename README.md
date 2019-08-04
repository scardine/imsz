# imsz

Get width and height from an image file reading as few bytes as possible. 

## Why?

A few years ago I answered a Stackoverflow question ["Get Image size WITHOUT loading image into memory"](https://stackoverflow.com/questions/15800704/get-image-size-without-loading-image-into-memory) (using Python). The canonical answer for dealing with images in Python is the PIL library but it is a huge library if you just want to get width and height. I rolled up my sleeves and wrote a small function to do just this. Over the years people sent patches impementing new image formats and refactoring the code until I could not recognize it anymore. I always wanted to tidy it up a bit, may be reimplement it as a C module...

# Am I rusty?

Over the last 10 years I've used Python for everything. This saturday afternoon I wanted to answer the question: "at my age, can I still learn a new computer language?". So I decided to try rust and this is the result. It was a pleasant surprise, if you are familiar with C/C++ rust is really easy to pick up - and I see some Python influence here and there. 

I don't expect it to be very idiomatic rust, it is my first rust project so be kind!

