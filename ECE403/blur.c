#include <stdio.h>
#include <stdlib.h>

#define BLUR_SIZE 3

// Function to load MNIST labels
void loadMNISTLabels(const char* filename, unsigned char** labels, size_t* numLabels);

// Function to load MNIST images
void loadMNISTImages(const char* filename, unsigned char*** images, size_t* numImages, size_t* width, size_t* height);

// Function to apply windowed averaging blur
void blurImage(const unsigned char* input, unsigned char* output, size_t width, size_t height);

// Function to save an image as a PGM file
void savePGMImage(const char* folder, const char* filename, const unsigned char* data, size_t width, size_t height);

int main() {
    // Specify the path to your MNIST labels file
    const char* mnistLabelsFilePath = "path/to/train-labels.idx1-ubyte";

    // Specify the path to your MNIST images file
    const char* mnistImagesFilePath = "path/to/train-images.idx3-ubyte";

    // Load MNIST labels
    unsigned char* mnistLabels;
    size_t numLabels;
    loadMNISTLabels(mnistLabelsFilePath, &mnistLabels, &numLabels);

    // Load MNIST images
    unsigned char** mnistImages;
    size_t numImages, imageWidth, imageHeight;
    loadMNISTImages(mnistImagesFilePath, &mnistImages, &numImages, &imageWidth, &imageHeight);

    // Process each image
    for (size_t imgIdx = 0; imgIdx < numImages; ++imgIdx) {
        // Check if the label is needed for further processing
        unsigned char label = mnistLabels[imgIdx];
        printf("Processing image with label: %d\n", label);

        // Allocate memory on the host
        size_t imageSize = imageWidth * imageHeight;
        unsigned char* h_input = mnistImages[imgIdx];
        unsigned char* h_output = (unsigned char*)malloc(imageSize * sizeof(unsigned char));

        // Apply windowed averaging blur
        blurImage(h_input, h_output, imageWidth, imageHeight);

        // Save the blurred image to a folder
        const char* outputFolder = "output_folder";
        char outputFilename[100];
        snprintf(outputFilename, sizeof(outputFilename), "blurred_image_%zu.pgm", imgIdx);
        savePGMImage(outputFolder, outputFilename, h_output, imageWidth, imageHeight);

        // Clean up for each image
        free(h_output);
    }

    // Clean up
    free(mnistLabels);
    for (size_t i = 0; i < numImages; ++i) {
        free(mnistImages[i]);
    }
    free(mnistImages);

    return 0;
}

void loadMNISTLabels(const char* filename, unsigned char** labels, size_t* numLabels) {
    FILE* file = fopen(filename, "rb");
    if (!file) {
        perror("Error: Could not open MNIST file");
        exit(EXIT_FAILURE);
    }

    // Read the magic number and label count
    uint32_t magicNumber, numLabelsRead;
    fread(&magicNumber, sizeof(uint32_t), 1, file);
    fread(&numLabelsRead, sizeof(uint32_t), 1, file);
    magicNumber = __builtin_bswap32(magicNumber);
    numLabelsRead = __builtin_bswap32(numLabelsRead);

    // Read labels
    *numLabels = numLabelsRead;
    *labels = (unsigned char*)malloc(numLabelsRead * sizeof(unsigned char));
    fread(*labels, sizeof(unsigned char), numLabelsRead, file);

    fclose(file);
}

void loadMNISTImages(const char* filename, unsigned char*** images, size_t* numImages, size_t* width, size_t* height) {
    FILE* file = fopen(filename, "rb");
    if (!file) {
        perror("Error: Could not open MNIST file");
        exit(EXIT_FAILURE);
    }

    // Read the magic number and image count
    uint32_t magicNumber, numImagesRead;
    fread(&magicNumber, sizeof(uint32_t), 1, file);
    fread(&numImagesRead, sizeof(uint32_t), 1, file);
    magicNumber = __builtin_bswap32(magicNumber);
    numImagesRead = __builtin_bswap32(numImagesRead);

    // Read image dimensions
    fread(width, sizeof(uint32_t), 1, file);
    fread(height, sizeof(uint32_t), 1, file);
    *width = __builtin_bswap32(*width);
    *height = __builtin_bswap32(*height);

    // Read image data
    *numImages = numImagesRead;
    *images = (unsigned char**)malloc(numImagesRead * sizeof(unsigned char*));
    for (size_t i = 0; i < numImagesRead; ++i) {
        (*images)[i] = (unsigned char*)malloc((*width) * (*height) * sizeof(unsigned char));
        fread((*images)[i], sizeof(unsigned char), (*width) * (*height), file);
    }

    fclose(file);
}

void blurImage(const unsigned char* input, unsigned char* output, size_t width, size_t height) {
    // Apply windowed averaging blur algorithm
    for (size_t i = 0; i < height; ++i) {
        for (size_t j = 0; j < width; ++j) {
            int sum = 0;
            int count = 0;

            for (int di = -BLUR_SIZE / 2; di <= BLUR_SIZE / 2; ++di) {
                for (int dj = -BLUR_SIZE / 2; dj <= BLUR_SIZE / 2; ++dj) {
                    int curRow = i + di;
                    int curCol = j + dj;

                    if (curRow >= 0 && curRow < (int)height && curCol >= 0 && curCol < (int)width) {
                        sum += input[curRow * width + curCol];
                        count++;
                    }
                }
            }

            output[i * width + j] = (unsigned char)(sum / count);
        }
    }
}

void savePGMImage(const char* folder, const char* filename, const unsigned char* data, size_t width, size_t height) {
    FILE* file = fopen(folder, "wb");
    if (!file) {
        perror("Error: Could not open file for writing");
        return;
    }

    // Write PGM header
    fprintf(file, "P5\n%zu %zu\n255\n", width, height);

    // Write image data
    fwrite(data, sizeof(unsigned char), width * height, file);

    fclose(file);
}
