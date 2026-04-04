#pragma once

#include <array>
#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <expected>
#include <span>
#include <variant>

enum class CanError : uint8_t {
  UnknownId,
  InvalidLength,
  InvalidData,
};

namespace detail {

template <typename T>
[[nodiscard]] constexpr T read_le(const uint8_t *d) noexcept {
  T v{};
  std::memcpy(&v, d, sizeof(T));
  if constexpr (std::endian::native == std::endian::big) v = std::byteswap(v);
  return v;
};

template <typename T>
[[nodiscard]] constexpr T read_be(const uint8_t *d) noexcept {
  T v{};
  std::memcpy(&v, d, sizeof(T));
  if constexpr (std::endian::native == std::endian::little) v = std::byteswap(v);
  return v;
};

template <typename T>
constexpr void write_le(uint8_t *d, T v) noexcept {
  if constexpr (std::endian::native == std::endian::big) v = std::byteswap(v);
  std::memcpy(d, &v, sizeof(T));
};

template <typename T>
constexpr void write_be(uint8_t *d, T v) noexcept {
  if constexpr (std::endian::native == std::endian::little) v = std::byteswap(v);
  std::memcpy(d, &v, sizeof(T));
};

} // namespace detail

enum class DriverHeartbeatCmd : uint8_t {
  Reboot = 2,
  Sync = 1,
  Noop = 0,
};

struct DriverHeartbeat {
};

enum class IoDebugTestEnum : uint8_t {
  Two = 2,
  One = 1,
};

struct IoDebug {
};

struct MotorCmd {
};

struct MotorStatus {
};

struct SensorSonars {
};

