// MIT LICENSE
//
// COPYRIGHT (R) 2025 ARNELIFY. AUTHOR: TARON SARKISYAN
//
// PERMISSION IS HEREBY GRANTED, FREE OF CHARGE, TO ANY PERSON OBTAINING A COPY
// OF THIS SOFTWARE AND ASSOCIATED DOCUMENTATION FILES (THE "SOFTWARE"), TO DEAL
// IN THE SOFTWARE WITHOUT RESTRICTION, INCLUDING WITHOUT LIMITATION THE RIGHTS
// TO USE, COPY, MODIFY, MERGE, PUBLISH, DISTRIBUTE, SUBLICENSE, AND/OR SELL
// COPIES OF THE SOFTWARE, AND TO PERMIT PERSONS TO WHOM THE SOFTWARE IS
// FURNISHED TO DO SO, SUBJECT TO THE FOLLOWING CONDITIONS:
//
// THE ABOVE COPYRIGHT NOTICE AND THIS PERMISSION NOTICE SHALL BE INCLUDED IN ALL
// COPIES OR SUBSTANTIAL PORTIONS OF THE SOFTWARE.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#ifndef ARNELIFY_SERVER_HTTP2_H
#define ARNELIFY_SERVER_HTTP2_H

#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef void (*http2_handler_t)(int c_id, int c_stream_id, const char* c_ctx);
typedef void (*http2_logger_t)(int c_id, const char* level,
                               const char* message);

void http2_add_header(int c_stream_id, const char* c_key, const char* c_value);

int http2_create(const char* c_opts);

void http2_destroy(int c_id);

void http2_end(int c_stream_id);

void http2_logger(int c_id, http2_logger_t c_cb);

void http2_on(int c_id, const char* c_path, http2_handler_t c_cb);

void http2_push_bytes(int c_stream_id, const char* c_bytes, int c_bytes_len,
                      int c_is_attachment);

void http2_push_file(int c_stream_id, const char* c_file_path,
                     int c_is_attachment);

void http2_push_json(int c_stream_id, const char* c_json, int c_is_attachment);

void http2_set_code(int c_stream_id, int c_code);

void http2_set_compression(int c_stream_id, const char* c_compression);

void http2_set_headers(int c_stream_id, const char* c_headers_json);

void http2_start(int c_id);

void http2_stop(int c_id);

#ifdef __cplusplus
}
#endif

#endif