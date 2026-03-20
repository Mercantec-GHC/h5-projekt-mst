#include <microhttpd.h>
#include <stdio.h>
#include <string.h>
#include <sys/select.h>
#include <sys/socket.h>
#include <sys/types.h>

#define PORT 8888

int print_out_key(
    void* cls, enum MHD_ValueKind kind, const char* key, const char* value)
{
    printf("%s: %s\n", key, value);
    return MHD_YES;
}

int answer_to_connection(void* cls, struct MHD_Connection* connection,
    const char* url, const char* method, const char* version,
    const char* upload_data, size_t* upload_data_size, void** req_cls)
{
    const char* page = "<html><body>Hello, browser!</body></html>";
    struct MHD_Response* response;
    int ret;

    MHD_get_connection_values(connection,
        MHD_HEADER_KIND,
        (MHD_KeyValueIterator)&print_out_key,
        NULL);

    printf("New %s request for %s using version %s\n", method, url, version);

    response = MHD_create_response_from_buffer(
        strlen(page), (void*)page, MHD_RESPMEM_PERSISTENT);

    ret = (int)MHD_queue_response(connection, MHD_HTTP_OK, response);
    MHD_destroy_response(response);

    return ret;
}

int main(void)
{
    printf("hello\n");
    struct MHD_Daemon* daemon;

    daemon = MHD_start_daemon(MHD_USE_INTERNAL_POLLING_THREAD,
        PORT,
        NULL,
        NULL,
        (MHD_AccessHandlerCallback)&answer_to_connection,
        NULL,
        MHD_OPTION_END);

    if (NULL == daemon)
        return 1;
    getchar();

    MHD_stop_daemon(daemon);
    return 0;
}
