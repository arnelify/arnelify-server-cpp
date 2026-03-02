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

#ifndef ARNELIFY_SERVER_WS_H
#define ARNELIFY_SERVER_WS_H

#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef void (*ws_handler_t)(const int c_id, const int c_stream_id,
                             const char* c_ctx, const char* c_bytes,
                             const int c_bytes_len);

typedef void (*ws_logger_t)(const int c_id, const char* c_level,
                            const char* c_message);

void ws_close(const int c_stream_id);

int ws_create(const char* c_opts);

void ws_destroy(const int c_id);

void ws_logger(const int c_id, ws_logger_t c_cb);

void ws_on(const int c_id, const char* c_topic, ws_handler_t c_cb);

void ws_push(const int c_stream_id, const char* c_json, const char* c_bytes,
             const int c_bytes_len);

void ws_push_bytes(const int c_stream_id, const char* c_bytes,
                   const int c_bytes_len);

void ws_push_json(const int c_stream_id, const char* c_json);

void ws_start(const int c_id);

void ws_stop(const int c_id);

#ifdef __cplusplus
}
#endif

#endif