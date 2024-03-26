#include "mraa_beaglebone_pinmap.h"
#include <signal.h>
#include <time.h>
#define LCD_ADDR 		 0x3E
#define bool int
#define true 1
#define false 0

int ADC_Chan = 7;       // Default ADC Channel on MCP3208

int period = 500000000; //start at 2 Hz
float duty_cycle = 0.2;

float preset_temp = 28.0;//preset temperature

mraa_i2c_context i2cp;
mraa_i2c_context I2Chandle;
mraa_gpio_context MCP3208_DIN;
mraa_gpio_context MCP3208_DOUT;
mraa_gpio_context MCP3208_CLK;
mraa_gpio_context MCP3208_CS;
mraa_i2c_context B1_pin, B2_pin, B3_pin;

char pwm_per[] ="/sys/class/pwm/pwm-0:1/period";
char pwm_pulse_w[] ="/sys/class/pwm/pwm-0:1/duty_cycle";
char pwm_enable[] ="/sys/class/pwm/pwm-0:1/enable";
char pwm_chip_un[] = "/sys/class/pwm/pwmchip0/unexport";
char pwm_chip_ex[] = "/sys/class/pwm/pwmchip0/export";
FILE *ppntr,*wpntr,*epntr;



void home_LCD (void)
{
        uint8_t buf[2] = {0x00,0x02};
        mraa_i2c_write(i2cp, buf, 2);  //Set to Home
}
void home2_LCD (void)
{
        uint8_t buf[] = {0x00,0x02,0xC0};
        mraa_i2c_write(i2cp, buf, 3);  //Set to Start of 2nd line 0X40 
}
void LCD_Print2 (char* str)
{
        uint8_t buf[80]={0};  // Set Buffer to all Null
        int i = 0, strl;      
        home2_LCD ();
        buf[i] = 0x40;  //register for display
        i++;
        strl = strlen((char*)str);
        for (int j = 0; j < strl; j++)
        {
               buf[i] = str[j];
               i++;

        }
         mraa_i2c_write(i2cp, buf, i);
}
void LCD_Print (uint8_t* str)
{
        uint8_t buf[80]={0};   // Set Buffer to all Null
	uint8_t buf1[2]={0x00,0x80};	
        int32_t i = 0, strl, j=0; 
        buf[i] = 0x40;  //register for display
        i++;
        strl = strlen((char*)str);
        for (j = 0; j < strl; j++)
        {
                buf[i] = str[j];
		i++;
  
        }
        
	mraa_i2c_write(i2cp, buf1, 2);
        mraa_i2c_write(i2cp, buf, i);
}

void LCD_init (void)
{
   uint8_t init1[2] = {0x00,0x38};
   uint8_t init2[8] = {0x00, 0x39, 0x14,0x74,0x54,0x6f,0x0c,0x01};
   // 2 lines 8 bit 3.3V Version
    mraa_i2c_write(i2cp, init1, 2);
    mraa_i2c_write(i2cp, init2,8);  //Function Set
}

void clear_LCD (void)
{
        uint8_t buf[2] = {0x00,0x01};
        mraa_i2c_write(i2cp, buf, 2);  //Clear Display
}


int GetMCP3208 (int Channel)
{
	int i;
	int val;
	
	mraa_gpio_write (MCP3208_DIN, 0);
	mraa_gpio_write (MCP3208_CLK, 0);
	mraa_gpio_write (MCP3208_CS, 0);

	Channel = Channel | 0x18;
	for (i = 0; i < 5; i ++)
	{
		if (Channel & 0x10)
		{
			mraa_gpio_write (MCP3208_DIN, 1);
		}
		else
		{
			mraa_gpio_write (MCP3208_DIN, 0);
		}
		Channel <<= 1;

		mraa_gpio_write (MCP3208_CLK, 0);
		mraa_gpio_write (MCP3208_CLK, 1);
	}

	mraa_gpio_write (MCP3208_CLK, 0);
	mraa_gpio_write (MCP3208_CLK, 1);

	mraa_gpio_write (MCP3208_CLK, 0);
	mraa_gpio_write (MCP3208_CLK, 1);

	val = 0;
	for (i = 0; i < 12; i ++)
	{
		mraa_gpio_write (MCP3208_CLK, 0);
		mraa_gpio_write (MCP3208_CLK, 1);
		
		val = (val << 1) | ((int) mraa_gpio_read (MCP3208_DOUT));	
	}
	
	mraa_gpio_write (MCP3208_CS, 1);
	mraa_gpio_write (MCP3208_DIN, 0);
	mraa_gpio_write (MCP3208_CLK, 0);

	return val;
}

int close_pwm()
{
	epntr = fopen(pwm_enable,"w");
	if (epntr != NULL)
	{
		fprintf(epntr,"0");
		fclose(epntr);
	}
	epntr = fopen(pwm_chip_un,"w");
	if (epntr != NULL)
	{
		fprintf(epntr,"0");
		fclose(epntr);
	}
}
int set_duty(float duty)
{
	int pulse_w;
	wpntr = fopen(pwm_pulse_w,"w");
	if (wpntr == NULL) 
	{
		printf("Can't set pwm pin pulse width, exiting\n");
		close_pwm();
		return MRAA_ERROR_UNSPECIFIED;
	}
	pulse_w = (int) period*duty;
	printf("Duty Cycle = %3.1f \%\n",duty*100);
	fprintf(wpntr,"%d",pulse_w);
	fclose(wpntr);
	return MRAA_SUCCESS;
}

int set_period(int per)
{
	int pulse_w;	
	ppntr = fopen(pwm_per,"w");
	if (ppntr == NULL) 
	{
		printf("Can't set pwm pin period, exiting\n");
		close_pwm();
		return MRAA_ERROR_UNSPECIFIED;
	}
	fprintf(ppntr,"%d",per);
	printf("\n done \n");
	fclose(ppntr);
	
	wpntr = fopen(pwm_pulse_w,"w");
	if (wpntr == NULL) 
	{
		printf("Can't set pwm pin pulse width, exiting\n");
		close_pwm();
		return MRAA_ERROR_UNSPECIFIED;
	}
	pulse_w = (int) per*duty_cycle;
	printf("Period = %d msec\n", per/1000000);
	fprintf(wpntr,"%d",pulse_w);
	fclose(wpntr);
	return MRAA_SUCCESS;
}

void exit_signal(int signum)
{
	printf("\nExiting PWM Program \n");
	close_pwm();
	exit(signum);
}
int setup_pwm()
{
	ppntr = fopen(pwm_per,"r");
	if (ppntr != NULL)
	{
		fclose(ppntr);
		printf("PWM already enabled\n");
	}
	else
	{
		epntr=fopen(pwm_chip_ex,"w");
		printf("Enabling PWM\n");
		fprintf(epntr,"1");
		fclose(epntr);
	}
	// set the PWM period in uS
	ppntr = fopen(pwm_per,"w");
	if (ppntr == NULL) 
	{
		printf("Can't set pwm pin period, exiting\n");
		close_pwm();
		return MRAA_ERROR_UNSPECIFIED;
	}
	fprintf(ppntr,"%d",period);
	printf("\n done \n");
	fclose(ppntr);

	wpntr = fopen(pwm_pulse_w,"w");
	if (wpntr == NULL) 
	{
		printf("Can't set pwm pin pulse width, exiting\n");
		close_pwm();
		return MRAA_ERROR_UNSPECIFIED;
	}
	fprintf(wpntr,"%d",(int)period*0.1);
	fclose(wpntr);

	epntr = fopen(pwm_enable,"w");
	if (epntr == NULL) 
	{
		printf("Can't enable pwm pin, exiting\n");
		close_pwm();
		return MRAA_ERROR_UNSPECIFIED;
	}
	fprintf(epntr,"%d",1);
	fclose(epntr);
}


int main()
{
	char buf [20];
	
	mraa_gpio_context button1, button2, button3, relay;
	mraa_init();
	
	i2cp = mraa_i2c_init_raw (I2CP_BUS);
	if (!i2cp) { // The bus is not there 
		printf("I2C Bus 2 is not available. Please check /dev/i2c-2 \n");
		printf(" -> Exiting program\n");
		mraa_i2c_stop(i2cp);
		mraa_deinit();
		return EXIT_FAILURE;
	}

	mraa_i2c_frequency (i2cp, MRAA_I2C_STD);
	mraa_i2c_address(i2cp, LCD_ADDR);
	
	// Initialize GPIO pins for SPI communication
	MCP3208_DIN = mraa_gpio_init (SPI_MOSI_PIN);
	MCP3208_DOUT = mraa_gpio_init (SPI_MISO_PIN);
	MCP3208_CLK = mraa_gpio_init (SPI_CLK_PIN);
	MCP3208_CS = mraa_gpio_init (SPI_CS0_PIN);
    sleep(1); // Need a delay before setting the direction.	
	
	// Set GPIO directions
	mraa_gpio_dir(MCP3208_DIN, MRAA_GPIO_OUT_HIGH);
	mraa_gpio_dir(MCP3208_DOUT, MRAA_GPIO_IN);
	mraa_gpio_dir(MCP3208_CLK, MRAA_GPIO_OUT);
	mraa_gpio_dir(MCP3208_CS, MRAA_GPIO_OUT);
	
	button1 = mraa_gpio_init(B1);
	button2 = mraa_gpio_init(B2);
	button3 = mraa_gpio_init(B3);
	relay = mraa_gpio_init(RELAY_PIN);
	sleep(1);
	
	LCD_init();
	
	bool alarm = false;
	long start;
	long endwait1min;
	long endwait2min;
	time_t start_time = 0;
	signal(SIGINT, exit_signal);
	
	setup_pwm();
	set_duty(1);
	printf("the period is %d\n", period);
	while (1) {

		int B1_val = mraa_gpio_read(button1);
		int B2_val = mraa_gpio_read(button2);
		int B3_val = mraa_gpio_read(button3);

		float current_temp = ((((float) GetMCP3208 (ADC_Chan)) * 3300 / 4096)-500)/10;
		
		// First line for the current temperature from sensor
		sprintf (buf, "Current: %3.1f%cC  ", current_temp, 0xDF);
		LCD_Print((uint8_t*)buf);
		
		// Press button1 to increase temperature
		if(!B1_val && preset_temp <= 36.0) {
			preset_temp += 1.0;
			usleep(10000);
		}
		// Press button2 to decrease temperature
		if(!B2_val && preset_temp >= 18.0) {
			preset_temp -= 1.0;
			usleep(10000);
		}

		// Second line for the preset temperature
		sprintf(buf, "Preset: %3.1f%cC  ", preset_temp, 0xDF);
		LCD_Print2(buf);
		sleep(1);

		// Press button3 to restart the temperature detect when not alarm
		if (!B3_val) {
			alarm = false;
			
			set_period(500000000);
			set_duty(1);

			mraa_gpio_write(relay, 1);
			preset_temp = current_temp;
			start_time = 0;
			printf("reset the system!\n");
		}	
		
		// When the current temperature is higher than the preset temperature, start the alarm
		if (current_temp > preset_temp && alarm == false) {
			alarm = true;
			set_duty(0.2);
			start_time = time(NULL);
			printf("alarm!\n");
		}
		
		if (alarm) {
			time_t current_time = time(NULL);
			double elapsed_time = difftime(current_time, start_time);
			printf("%f time already!\n", elapsed_time);
			
			// When the alarm continues over 1 min change the period
			if (elapsed_time >= 60 && elapsed_time < 120 && alarm) {
				set_period(100000000);
				printf("1 min already!\n");
			}
			// When the alarm continues over 2 mins start the delay
			else if (elapsed_time >= 120 && alarm) {
				mraa_gpio_write(relay, 0);
				printf("2 min already!\n");
				sleep(2);
			}
		}

	}
	close_pwm();
	return MRAA_SUCCESS;
}
