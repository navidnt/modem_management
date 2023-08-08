#include <errno.h>
#include <fcntl.h>
#include <string.h>
#include <termios.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <string.h>


#define SOCKET1_PATH "/data/local/tmp/sock3"
#define SOCKET2_PATH "/data/local/tmp/sock4"
#define BUFFER_SIZE 1024

int set_interface_attribs (int fd, int speed, int parity)
{
    struct termios tty;
    
    tcgetattr(fd, &tty);

    cfsetospeed(&tty, (speed_t)speed);
    cfsetispeed(&tty, (speed_t)speed);

    tty.c_cflag &= ~PARENB; // Clear parity bit, disabling parity (most common)
    tty.c_cflag &= ~CSTOPB; // Clear stop field, only one stop bit used in
    // communication (most common)
    tty.c_cflag &= ~CSIZE; // Clear all bits that set the data size
    tty.c_cflag |= CS8;    // 8 bits per byte (most common)
    tty.c_cflag &= ~CRTSCTS; // Disable RTS/CTS hardware flow control (most common)
    tty.c_cflag |= CREAD | CLOCAL; // Turn on READ & ignore ctrl lines (CLOCAL = 1)
    tty.c_lflag &= ~ICANON;
    tty.c_lflag &= ~ECHO;   // Disable echo
    tty.c_lflag &= ~ECHOE;  // Disable erasure
    tty.c_lflag &= ~ECHONL; // Disable new-line echo
    tty.c_lflag &= ~ISIG;   // Disable interpretation of INTR, QUIT and SUSP
    tty.c_iflag &= ~(IXON | IXOFF | IXANY); // Turn off s/w flow ctrl
    tty.c_iflag &= ~(IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR |
                    ICRNL); // Disable any special handling of received bytes
    tty.c_oflag &= ~OPOST; // Prevent special interpretation of output bytes (e.g.
    // newline chars)
    tty.c_oflag &= ~ONLCR; // Prevent conversion of newline to carriage
                            // return/line feed
    // tty.c_oflag &= ~OXTABS; // Prevent conversion of tabs to spaces
    // (NOT PRESENT ON LINUX) tty.c_oflag &= ~ONOEOT; // Prevent removal
    // of C-d chars (0x004) in output (NOT PRESENT ON LINUX)
    tty.c_cc[VTIME] = 10; // Wait for up to 1s (10 deciseconds), returning as soon
    // as any data is received.
    tty.c_cc[VMIN] = 0;
    tcsetattr(fd, TCSANOW, &tty);
    return 1;
}



int main(){

    while (1){

        const char* filename = SOCKET1_PATH;

        // Delete the file
        if (remove(filename) == 0) {
            printf("File deleted successfully: %s\n", filename);
        } else {
            perror("Failed to delete the file");
        }


        int sock1;
        struct sockaddr_un server_addr, client_addr;
        char buffer[BUFFER_SIZE];

        sock1 = socket(AF_UNIX, SOCK_DGRAM, 0);
        if (sock1 == -1) {
            perror("socket");
            exit(EXIT_FAILURE);
        }

        // Bind the socket
        memset(&server_addr, 0, sizeof(struct sockaddr_un));
        server_addr.sun_family = AF_UNIX;
        strncpy(server_addr.sun_path, SOCKET1_PATH, sizeof(server_addr.sun_path) - 1);

        if (bind(sock1, (struct sockaddr*)&server_addr, sizeof(struct sockaddr_un)) == -1) {
            perror("bind");
            exit(EXIT_FAILURE);
        }

        printf("Listening on socket: %s\n", SOCKET1_PATH);

        // Receive messages
        socklen_t client_addr_len = sizeof(struct sockaddr_un);
        ssize_t num_bytes_received = recvfrom(sock1, buffer, BUFFER_SIZE - 1, 0,
                                              (struct sockaddr*)&client_addr, &client_addr_len);
        if (num_bytes_received == -1) {
            perror("recvfrom");
            exit(EXIT_FAILURE);
        }

        buffer[num_bytes_received] = '\n';
        buffer[num_bytes_received + 1] = '\0';
        printf("Received %zd bytes from client: %s\n", num_bytes_received, buffer);

        // Close the socket
        close(sock1);	



        char *msg = buffer;
        //msg[num_bytes_received + 1] = '\n';
        printf("msg = %s etmam  num_bytes_received: %d  msg_last: %c  \n", msg, num_bytes_received, msg[20]);
        char *portname = "/dev/ttyUSB3";

        int fd = open (portname, O_RDWR);

        set_interface_attribs (fd, B115200, 0);  // set speed to 115,200 bps, 8n1 (no parity)
        write (fd, msg, num_bytes_received + 2);           // send 7 character greeting

        usleep ((15 + 25) * 1000);             // sleep enough to transmit the 7 plus
                                            // receive 25:  approx 100 uS per char transmit
        char buf [100];
        int n = read (fd, buf, sizeof buf);  // read up to 100 characters if ready to read

        printf("Num Bytes Recv	: %d", num_bytes_received);
        close(fd);




        int sock2;
        struct sockaddr_un dest_addr;
        char *message = buf;
       // char *message = "hello";
	printf("message len: %d", strlen(message));
        /*for(int  i = 0; i < num_bytes_received + 1; i++){
            message[i] = msg[i];
        }*/

        sock2 = socket(AF_UNIX, SOCK_DGRAM, 0);
        if (sock2 == -1) {
            perror("socket");
            exit(EXIT_FAILURE);
        }

			        // Set the destination address
        memset(&dest_addr, 0, sizeof(struct sockaddr_un));
        dest_addr.sun_family = AF_UNIX;
        strncpy(dest_addr.sun_path, SOCKET2_PATH, sizeof(dest_addr.sun_path) - 1);
        ssize_t num_bytes_sent = sendto(sock2, message, strlen(message), 0,
                                        (struct sockaddr*)&dest_addr, sizeof(struct sockaddr_un));
        if (num_bytes_sent == -1) {
            perror("sendto");
            exit(EXIT_FAILURE);
        }

	printf("Sent %zd bytes: %s\n", num_bytes_sent, message);

        // Close the socket
        close(sock2);
    }
}
		
