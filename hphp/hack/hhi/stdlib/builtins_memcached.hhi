<?hh /* -*- php -*- */
/**
 * Copyright (c) 2014, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree.
 *
 */

<<__PHPStdLib>>
class Memcached {
  const int OPT_COMPRESSION = 0;
  const int OPT_SERIALIZER = 0;
  const int SERIALIZER_PHP = 0;
  const int SERIALIZER_IGBINARY = 0;
  const int SERIALIZER_JSON = 0;
  const int OPT_PREFIX_KEY = 0;
  const int OPT_HASH = 0;
  const int HASH_DEFAULT = 0;
  const int HASH_MD5 = 0;
  const int HASH_CRC = 0;
  const int HASH_FNV1_64 = 0;
  const int HASH_FNV1A_64 = 0;
  const int HASH_FNV1_32 = 0;
  const int HASH_FNV1A_32 = 0;
  const int HASH_HSIEH = 0;
  const int HASH_MURMUR = 0;
  const int OPT_DISTRIBUTION = 0;
  const int DISTRIBUTION_MODULA = 0;
  const int DISTRIBUTION_CONSISTENT = 0;
  const int OPT_LIBKETAMA_COMPATIBLE = 0;
  const int OPT_LIBKETAMA_HASH = 0;
  const bool GET_ERROR_RETURN_VALUE = false;
  const int LIBMEMCACHED_VERSION_HEX = 0;
  const int OPT_BUFFER_WRITES = 0;
  const int OPT_BINARY_PROTOCOL = 0;
  const int OPT_NO_BLOCK = 0;
  const int OPT_TCP_NODELAY = 0;
  const int OPT_SOCKET_SEND_SIZE = 0;
  const int OPT_SOCKET_RECV_SIZE = 0;
  const int OPT_CONNECT_TIMEOUT = 0;
  const int OPT_RETRY_TIMEOUT = 0;
  const int OPT_SEND_TIMEOUT = 0;
  const int OPT_RECV_TIMEOUT = 0;
  const int OPT_POLL_TIMEOUT = 0;
  const int OPT_CACHE_LOOKUPS = 0;
  const int OPT_SERVER_FAILURE_LIMIT = 0;
  const int HAVE_IGBINARY = 0;
  const int HAVE_JSON = 0;
  const int GET_PRESERVE_ORDER = 0;
  const int RES_SUCCESS = 0;
  const int RES_FAILURE = 0;
  const int RES_HOST_LOOKUP_FAILURE = 0;
  const int RES_UNKNOWN_READ_FAILURE = 0;
  const int RES_PROTOCOL_ERROR = 0;
  const int RES_CLIENT_ERROR = 0;
  const int RES_SERVER_ERROR = 0;
  const int RES_WRITE_FAILURE = 0;
  const int RES_DATA_EXISTS = 0;
  const int RES_NOTSTORED = 0;
  const int RES_NOTFOUND = 0;
  const int RES_PARTIAL_READ = 0;
  const int RES_SOME_ERRORS = 0;
  const int RES_NO_SERVERS = 0;
  const int RES_END = 0;
  const int RES_ERRNO = 0;
  const int RES_BUFFERED = 0;
  const int RES_TIMEOUT = 0;
  const int RES_BAD_KEY_PROVIDED = 0;
  const int RES_CONNECTION_SOCKET_CREATE_FAILURE = 0;
  const int RES_PAYLOAD_FAILURE = 0;

  public function __construct($persistent_id = null);
  public function add($key, $value, int $expiration = 0);
  public function addByKey(string $server_key, string $key, $value, int $expiration = 0);
  public function addServer(string $host, int $port, int $weight = 0);
  public function addServers($servers);
  public function append($key, $value);
  public function appendByKey(string $server_key, string $key, string $value);
  public function cas(float $cas_token, string $key, $value, int $expiration = 0);
  public function casByKey(float $cas_token, string $server_key, string $key, $value, int $expiration = 0);
  public function decrement(string $key, int $offset = 1);
  public function delete(string $key, int $time = 0);
  public function deleteByKey(string $server_key, string $key, int $time = 0);
  public function deleteMulti(varray<string> $keys, int $time = 0): mixed;
  public function deleteMultiByKey(string $server_key, varray<string> $keys, int $time = 0): mixed;
  public function fetch();
  public function fetchAll();
  public function flush(int $delay = 0);
  public function get($key, $cache_cb = null);
  public function getByKey(string $server_key, string $key, $cache_cb = null);
  public function getWithCasToken($key, $cache_cb, inout $cas_token);
  public function getByKeyWithCasToken(string $server_key, string $key, $cache_cb, inout $cas_token);
  public function getDelayed($keys, $with_cas = false, $value_cb = null);
  public function getDelayedByKey(string $server_key, $keys, bool $with_cas = false, $value_cb = null);
  public function getMulti($keys, int $flags = 0);
  public function getMultiByKey(string $server_key, $keys, int $flags = 0);
  public function getMultiWithCasTokens($keys, inout $cas_tokens, int $flags = 0);
  public function getMultiByKeyWithCasTokens(string $server_key, $keys, inout $cas_tokens, int $flags = 0);
  public function getOption(int $option);
  public function getResultCode();
  public function getResultMessage();
  public function getServerByKey(string $server_key);
  public function getServerList();
  public function getStats();
  public function getVersion();
  public function increment(string $key, int $offset = 1);
  public function prepend($key, $value);
  public function prependByKey(string $server_key, string $key, string $value);
  public function replace($key, $value, int $expiration = 0);
  public function replaceByKey(string $server_key, string $key, $value, int $expiration = 0);
  public function set($key, $value, int $expiration = 0);
  public function setByKey(string $server_key, string $key, $value, int $expiration = 0);
  public function setMulti($items, int $expiration = 0);
  public function setMultiByKey(string $server_key, $items, int $expiration = 0);
  public function setOption(int $option, $value);
  public function touch(string $key, int $expiration = 0): bool;
  public function touchByKey(string $server_key, string $key, int $expiration = 0): bool;
}
