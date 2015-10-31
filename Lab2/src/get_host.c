#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>

#include <netinet/tcp.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <sys/select.h>
#include <netinet/in.h>
#include <netdb.h>
#include <arpa/inet.h>

char* get_host(char* host )
{
    struct addrinfo info;
    struct addrinfo *result;

    memset(&info, 0, sizeof(struct addrinfo));
    info.ai_family = AF_UNSPEC;
    info.ai_socktype = SOCK_STREAM;
    info.ai_protocol = 0;
    info.ai_flags = 0;

    if (getaddrinfo(host, "80", &info, &result) != 0) { return ""; }

    struct sockaddr_in *addr;
    addr = (struct sockaddr_in *)result->ai_addr;

    freeaddrinfo(result);

    return inet_ntoa((struct in_addr)addr->sin_addr);
}
