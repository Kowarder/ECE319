#include <iostream>
#include <fstream>
#include <vector>
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <time.h>
#include <cmath>
#include <string.h>

using namespace std;

const int input_node_num = 784 ;    //input node number
const int hidden_node_num = 100 ;   //hidden node number
const int output_node_num = 10 ;    //output node number
const double learning_rate = 0.01 ; //the learning rates

int input_layer[input_node_num] ;   //input layer
double hidden_layer_output[hidden_node_num] ;   //hidden layer
int output_layer_target[output_node_num] ;  //output layer target number
double output_layer_real[output_node_num] ; //output real number


double input_hidden_weight[input_node_num][hidden_node_num] ;   //input layer-> hidden layer weight
double hidden_output_weight[hidden_node_num][output_node_num] ; //hidden layer -> output layer weight

double hidden_bias[hidden_node_num] ;   //hidden layer bias
double output_bias[output_node_num] ;   //output layer bias

//calculate the delta change which could reduce the calculate times
double hidden_delta[hidden_node_num] ;  //hidden layer delta
double output_delta[output_node_num] ;  //output layer delta

double relu(double x)//active function relu
{
    return max(0.0, x);
}

double sigmod(double x)     //active function sigmod
{
    return 1.0 / (1.0 + exp(-x));
}

void forward_propagation_relu()  //forward propagation using relu
{
    for(int i = 0; i < hidden_node_num; i++)
    {
        double relu_sum = 0.0 ;
        for(int j = 0; j < input_node_num; j++)
        {
            relu_sum += input_layer[j] * input_hidden_weight[j][i];
        }
        double x = relu_sum + hidden_bias[i];
        hidden_layer_output[i] = relu(x);
    }

    for(int i = 0; i < output_node_num; i++)
    {
        double relu_sum = 0.0 ;
        for(int j = 0; j < hidden_node_num; j++)
        {
            relu_sum += hidden_layer_output[j] * hidden_output_weight[j][i] ;
        }
        double x = relu_sum + output_bias[i];
        output_layer_real[i] = relu(x) ;
    }
}



void forward_propagation()  //forward propagation using sigmod
{
    for(int i = 0; i < hidden_node_num; i++)
    {
        double sigmod_sum = 0.0 ;
        for(int j = 0; j < input_node_num; j++)
        {
            sigmod_sum += input_layer[j] * input_hidden_weight[j][i];
        }
        double x = sigmod_sum + hidden_bias[i];
        hidden_layer_output[i] = sigmod(x);
    }

    for(int i = 0; i < output_node_num; i++)
    {
        double sigmod_sum = 0.0 ;
        for(int j = 0; j < hidden_node_num; j++)
        {
            sigmod_sum += hidden_layer_output[j] * hidden_output_weight[j][i] ;
        }
        double x = sigmod_sum + output_bias[i];
        output_layer_real[i] = sigmod(x) ;
    }
}


void backward_propagation()     //backward propagation
{
    //update output layer bias
    for(int i = 0; i < output_node_num; i++)
    {
        double delta_bias = 0.0 ;
        delta_bias = (output_layer_real[i] - output_layer_target[i]) * output_layer_real[i] * (1.0 - output_layer_real[i]);
        output_bias[i] = output_bias[i] - (learning_rate * delta_bias);
    }
    //update hidden layer -> output layer weight
    for(int i = 0; i < hidden_node_num; i++)
    {
        for(int j = 0; j < output_node_num; j++)
        {
           hidden_output_weight[i][j] =  hidden_output_weight[i][j] - (learning_rate * (output_layer_real[j] - output_layer_target[j]) * output_layer_real[j] * (1 - output_layer_real[j]) * hidden_layer_output[i]);
        }
    }
    //update hidden layer bias
    for(int i = 0; i < hidden_node_num; i++)
    {
        double delta_sum = 0.0 ;
        for(int j = 0 ;j < output_node_num; j++)
        {
            delta_sum = delta_sum + (output_layer_real[j] - output_layer_target[j]) * output_layer_real[j] * (1 - output_layer_real[j]) * hidden_output_weight[i][j];
        }
        hidden_bias[i] = hidden_bias[i] - learning_rate * delta_sum * hidden_layer_output[i] * (1 - hidden_layer_output[i]);
    }
    //update input -> hidden layer weight
    for(int i = 0; i < input_node_num; i++)
    {
        double delta_sum = 0.0 ;
        for(int j = 0; j < hidden_node_num; j++)
        {
            for(int k = 0; k < output_node_num; k++)
            {
                delta_sum = delta_sum + (output_layer_real[k] - output_layer_target[k]) * output_layer_real[k] * (1 - output_layer_real[k]) * hidden_output_weight[j][k];
            }
            delta_sum = delta_sum * hidden_layer_output[j] * (1 - hidden_layer_output[j]) * input_layer[i];
            input_hidden_weight[i][j] = input_hidden_weight[i][j] - learning_rate * delta_sum;
        }
    }
}
double get_randnum()    //get random number from 0 to 1
{
    return (rand() % 1000 * 0.001 - 0.5);
}
double get_random(int min,int max)      //get the target random number
{
     return ( rand() % (max - min + 1) ) + min ;
}
void initialize_bpnet()     //initialize the net
{
    srand((int)time(0) + rand());
    for(int i = 0; i < input_node_num; i++)
    {
        for(int j = 0; j < hidden_node_num; j++)
        {
            input_hidden_weight[i][j] = get_randnum() ;
        }
    }

    for(int i = 0; i < hidden_node_num; i++)
    {
        for(int j = 0; j < output_node_num ; j++)
        {
            hidden_output_weight[i][j] = get_randnum();
        }
    }

    for(int i = 0; i < hidden_node_num ; i++)
    {
        hidden_bias[i] = get_randnum();
    }

    for(int i = 0; i < hidden_node_num ; i++)
    {
        output_bias[i] = get_randnum();
    }
}
void training() //training
{
    FILE* training_image;
    FILE* training_label;
    int training_count = 0;
    unsigned char image_buffer[784];    //save the image information
    unsigned char label_buffer[10];     //save the label

    training_image = fopen("./train-images.idx3-ubyte", "rb");
    training_label = fopen("./train-labels.idx1-ubyte", "rb");
    if (training_image == NULL || training_label == NULL) {
		cout << "open training file error" << endl;
		exit(0);
	}

	int head_info[1000];    //read the head information
	fread(head_info,1,16,training_image);   //read the 16byte head information
	fread(head_info,1,8,training_label);    //8byte head information

	cout<<"Training started..."<<endl;

	while(!feof(training_image) && !feof(training_label))
    {
        memset(image_buffer,0,784);
        memset(label_buffer,0,10);
        fread(image_buffer,1,784,training_image);   //read a image
        fread(label_buffer,1,1,training_label);     //read a label

        for(int i = 0; i < 784; i++)    //initialize the input data by judge the gray
        {
            if((unsigned int)image_buffer[i] < 128)
            {
                input_layer[i] = 0;
            }
            else
            {
                input_layer[i] = 1;
            }
        }

        int label_value = (unsigned int)label_buffer[0];    //get the label


        for(int i = 0; i < output_node_num; i++)
        {
            output_layer_target[i] = 0 ;
        }
        output_layer_target[label_value] = 1;   //set the right data in output

        //forward_propagation_relu();
        forward_propagation();  //forward propagation using sigmod
        backward_propagation(); //backward propagation

        training_count++;
        if (training_count % 1500 == 0)
        {
             cout << "training percent: " << (training_count / 60000.0) * 100.0<<"%"<< endl;
        }
    }
     cout<<"training complete"<<endl;
}
void testing()  //test
{
    FILE *testing_image;
	FILE *testing_label;
	testing_image = fopen("./t10k-images.idx3-ubyte", "rb");
	testing_label = fopen("./t10k-labels.idx1-ubyte", "rb");

    double test_num = 0.0 ;
    double test_success_num = 0.0 ;

	if (testing_image == NULL || testing_label == NULL)
    {
		cout << "open training file error!" << endl;
		exit(0);
	}

	unsigned char image_buffer[784];
	unsigned char label_buffer[1];

	int useless[1000];
	fread(useless, 1, 16, testing_image);
	fread(useless, 1, 8, testing_label);

	while (!feof(testing_image) && !feof(testing_label))
    {
		memset(image_buffer, 0, 784);
		memset(label_buffer, 0, 1);
		fread(image_buffer, 1, 784, testing_image);
		fread(label_buffer, 1, 1, testing_label);

		for (int i = 0; i < 784; i++)
        {
			if ((unsigned int)image_buffer[i] < 128)
			{
				input_layer[i] = 0;
			}
			else
            {
				input_layer[i] = 1;
			}
		}

		for (int k = 0; k < output_node_num; k++)
        {
			output_layer_target[k] = 0;
		}

		int target_value = (unsigned int)label_buffer[0];
		output_layer_target[target_value] = 1;

        //forward_propagation_relu();
        forward_propagation();

		double max_value = -99999;
		int max_index = 0;
		for (int k = 0; k < output_node_num; k++)   //find the max value's index at output layer
        {
			if (output_layer_real[k] > max_value)
			{
				max_value = output_layer_real[k];
				max_index = k;
			}
		}

		if (output_layer_target[max_index] == 1)    //judge it's same as target index
        {
			test_success_num ++;
		}

		test_num ++;

		if ((int)test_num % 1000 == 0)
        {
			cout << "testing number: " << test_num << " success number: " << test_success_num << endl;
		}

	}
	cout << endl;
	cout << "success percent: " << test_success_num / test_num << endl;
}
int main()
{
    initialize_bpnet();
    training();
    testing();
    system("pause");
    return 0;
}
