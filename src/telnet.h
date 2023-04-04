#ifndef TELNET_H
#define TELNET_H

#include <libtelnet.h>

typedef struct telnet_server {
    int port;
    int server_fd;
    struct sockaddr_in *server_addr;
    telnet_t *telnet;
} telnet_server_t;

struct telnet_server_t *telnet_server_init(int port, telnet_event_handler_t telnet_event_handler);

void telnet_server_run(struct telnet_server *server);

void telnet_server_cleanup(struct telnet_server *server);

#endif /* TELNET_H */
