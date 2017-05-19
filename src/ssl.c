#include <openssl/ssl.h>
#include <openssl/err.h>
#include <string.h>

static int is_init = 0;

int alpn_cb (SSL *ssl, const unsigned char **out,
                           unsigned char *outlen,
                           const unsigned char *in,
                           unsigned int inlen,
                           void *arg);

/// TODO
// complete re work - CLEAN THIS SHIT UP
// This asumes that there is one global context that
// determins the begining and end of ssl usage


// stuff to remember
// SSL_CTX_set_tmp_dh_callback
// SSL_CTX_set_min_proto_version and SSL_CTX_set_max_proto_version .

void* newctx(const char* cert_file, const char* key_file) {
    if ( ! is_init ) {
        OpenSSL_add_ssl_algorithms();
        is_init = 1;
    }

    const SSL_METHOD *method;
    SSL_CTX *ctx;

    method = TLS_server_method();

    ctx = SSL_CTX_new(method);
    if (!ctx) {
        //perror("Unable to create SSL context");
        //ERR_print_errors_fp(stderr);
        //exit(EXIT_FAILURE);
        return NULL;
    }

    SSL_CTX_set_ecdh_auto(ctx, 1);

    /* Set the key and cert */
    if (SSL_CTX_use_certificate_file(ctx, cert_file, SSL_FILETYPE_PEM) <= 0) {
        // ERR_print_errors_fp(stderr);
	    // exit(EXIT_FAILURE);1
        return NULL;
    }

    if (SSL_CTX_use_PrivateKey_file(ctx, key_file, SSL_FILETYPE_PEM) <= 0 ) {
        // ERR_print_errors_fp(stderr);
	    // exit(EXIT_FAILURE);
        return NULL;
    }

    SSL_CTX_set_alpn_select_cb(ctx, alpn_cb, NULL);

    SSL_CTX_set_min_proto_version(ctx, TLS1_2_VERSION);

    //long mask = SSL_CTX_get_options(ctx);
    
    // SSL* ssl = SSL_new(ctx);

    //long mask2 = SSL_get_options(ssl);

    return (void*) ctx;
}

void* newssl(void* ctx, int fd) {
    // WOLFSSL* ssl = wolfSSL_new(ctx);
    // wolfSSL_set_fd(ssl, fd);
    SSL* ssl = SSL_new(ctx);
    SSL_set_fd(ssl, fd);

    if (SSL_accept(ssl) <= 0) {
        return NULL;
    }

    return (void*) ssl;
}

// int sslaccept(void* ssl) {
//     // return wolfSSL_accept(ssl);
// }

int sslread(void* ssl, char* buf, int size) {
    // return wolfSSL_read(ssl, buf, size);
    return SSL_read(ssl, buf, size);
}

int sslwrite(void* ssl, char* buf, int size) {
    // return wolfSSL_write(ssl, buf, size);
    return SSL_write(ssl, buf, size);
}

// probably do something like this for all ERROR handling 1/0:ok/fail
// int sslshutdown(void* ssl) {
//     // if (wolfSSL_shutdown(ssl) == SSL_SUCCESS)
//     //     return 0;
//     // else
//     //     return 1;
// }

void deletessl(void* ssl) {
    SSL_free(ssl);
    // wolfSSL_free(ssl);
}

void deletectx(void* ctx) {
    SSL_CTX_free(ctx);
    EVP_cleanup();

    // wolfSSL_CTX_free(ctx);
    // // for now, assume that there is one ctx and when we delete it, we are done
    // wolfSSL_Cleanup();
}

int alpn_cb (SSL *ssl, const unsigned char **out,
                           unsigned char *outlen,
                           const unsigned char *in,
                           unsigned int inlen,
                           void *arg)
{
    // for now, only h2
    char* sp = strstr((const char*)in, "h2");
    if (!sp)
        return SSL_TLSEXT_ERR_NOACK;
    *out = (const unsigned char*)sp;
    *outlen = 2;
    return SSL_TLSEXT_ERR_OK;
}

// void enable_alpn(void* ctx) {

//     SSL_CTX_set_alpn_select_cb(ctx, alpn_cb, NULL);

// }

