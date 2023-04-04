#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <libtelnet.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>

#define PORT 2323

void handle_telnet_event(telnet_t *telnet, telnet_event_t *ev, void *user_data) {
  const char *msg;
  size_t len;

  switch (ev->type) {
    case TELNET_EV_DATA:
      msg = ev->data.buffer;
      len = ev->data.size;

      // Handle incoming data
      printf("Received data: %.*s\n", (int)len, msg);

      // Send response
      telnet_send(telnet, msg, len);
      break;

    case TELNET_EV_SEND:
      msg = ev->data.buffer;
      len = ev->data.size;

      // Send data to client
      send(*(int*)user_data, msg, len, 0);
      break;

    case TELNET_EV_ERROR:
      // Handle error event
      printf("Error: %s\n", ev->error.msg);
      break;

    default:
      // Ignore other events
      break;
  }
}

int main() {
  int server_fd, client_fd;
  struct sockaddr_in server_addr, client_addr;
  socklen_t client_len;
  telnet_t *telnet;

  // Create server socket
  if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) == -1) {
    perror("socket");
    exit(EXIT_FAILURE);
  }

  // Bind server socket to port 23
  memset(&server_addr, 0, sizeof(server_addr));
  server_addr.sin_family = AF_INET;
  server_addr.sin_addr.s_addr = htonl(INADDR_ANY);
  server_addr.sin_port = htons(PORT);
  if (bind(server_fd, (struct sockaddr *)&server_addr, sizeof(server_addr)) == -1) {
    perror("bind");
    exit(EXIT_FAILURE);
  }

  // Listen for incoming connections
  if (listen(server_fd, 10) == -1) {
    perror("listen");
    exit(EXIT_FAILURE);
  }

  // Wait for incoming connections and handle them
  while (1) {
    printf("Waiting for incoming connections...\n");
    client_len = sizeof(client_addr);
    if ((client_fd = accept(server_fd, (struct sockaddr *)&client_addr, &client_len)) == -1) {
      perror("accept");
      continue;
    }

    printf("New client connected: %s:%d\n", inet_ntoa(client_addr.sin_addr), ntohs(client_addr.sin_port));

    // Initialize telnet session
    telnet = telnet_init(NULL, handle_telnet_event, 0, &client_fd);

    // Send welcome message
    telnet_send(telnet, "Welcome to my Telnet server!\r\n", strlen("Welcome to my Telnet server!\r\n"));

    // Wait for data from client
    char buffer[1024];
    int bytes_read;
    while ((bytes_read = recv(client_fd, buffer, sizeof(buffer), 0)) > 0) {
      // Process incoming data using libtelnet
      telnet_recv(telnet, buffer, bytes_read);
    }

    // Clean up telnet session
    telnet_free(telnet);

    // Close client socket
    close(client_fd);
  }

  // Close server socket
  close(server_fd);

  return 0;
}