# rlight - right light or rust light.

###### Use a camera as a light sensor to automatically adjust the brightness of your screen in GNU/Linux.
###### Make the light right for you with an [adaptive sensitivity]() and an [advanced customisation]().
-----------

## Adaptive sensitivity
If rlight's suggested brightness doesn't suit you, change it manually. 
The program will detect it and change the sensitivity. 
It will take some time, but eventually you won't have to change the brightness manually.

## Advanced customisation
When you run the program for the first time,
it will generate a config file in the default config folder
for your operating system(~/.config/rlight in my case).

These are the options and their description:
- camera - (built in camera in laptops is usually 0) index of the camera that will be used.
- delay - (in seconds) waiting time between changing the brightness.
- set_brightness_cmd - (Will be run this way: set_brightness_cmd brightness) a command that will be used to set the brightness. 
- set_brightness_cmd - (Should return a number) a command that will be used to get the brightness. 
- light_sensetivity - sensitivity used when the average brightness is between 0 and 85.
- mid_sensetivity - sensitivity used when the average brightness is between 85 and 170.
- dark_sensetivity - sensitivity used when the average brightness is between 170 and 255.
- adaptive_sensetivity - described [here]().
- learning_coefficient - The bigger it is, the more serious rlight takes manual brightness changes, be careful!
- step - 1 To check all pixels. The bigger is the step, the fewer pixels are being checked.
