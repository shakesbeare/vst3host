#include <cstring>
#include <iostream>
#include <string>
#include <strings.h>

#include <sys/socket.h>
#include <netdb.h>
#include <arpa/inet.h>

class WriteSocket {
    const char* host {};
    const short port {};
    int sock_desc {};

    struct sockaddr_in server;
    struct in_addr ipv4addr;
    struct hostent *hp;

public:
    WriteSocket(const char* host, short port)
    :host { host }, port { port } {
        printf("constructor\n");
        sock_desc = socket(AF_INET, SOCK_DGRAM, 0);
        server.sin_family = AF_INET;

        inet_pton(AF_INET, this->host, &ipv4addr);
        hp = gethostbyaddr(&ipv4addr, sizeof ipv4addr, AF_INET);

        bcopy(hp->h_addr, &(server.sin_addr.s_addr), hp->h_length);
        server.sin_port = htons(port);

        /* int res = connect(sock_desc, (const sockaddr *)&server, sizeof(server)); */
        /* if (res == -1) { */
        /*     // an error has occurred */
        /*     throw std::runtime_error("Failed to connect to addr"); */
        /* } */
    }

    ~WriteSocket() {
        int shutdown_res = shutdown(sock_desc, SHUT_RDWR);
        if (shutdown_res == -1) {
            std::printf("Failed to shutdown socket you suck. Destructors aren't allowed to throw for some reason\n");
        }
    }

    void send_message(std::string message) {
        char* m_cstr = (char*)message.c_str();
        send(sock_desc, m_cstr, strlen(m_cstr), 0);
    }
};

int main() {
    printf("progr start\n");
    const char* host = "0.0.0.0";
    WriteSocket sock { host, 3000 };
}
