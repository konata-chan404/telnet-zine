#include <stdio.h>
#include "telnet.h"

#define PORT 2323

int main() {
  telnet_server_t *server = telnet_server_init(PORT, NULL);
  telnet_server_run(server);
  telnet_server_cleanup(server);
}