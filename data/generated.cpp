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

[[nodiscard]] constexpr std::expected<DriverHeartbeatCmd, CanError>
driver_heartbeat_cmd_from_raw(uint8_t v) noexcept {
  switch (v) {
    case 2: return DriverHeartbeatCmd::Reboot;
    case 1: return DriverHeartbeatCmd::Sync;
    case 0: return DriverHeartbeatCmd::Noop;
    default: return std::unexpected(CanError::InvalidData);
  };
};

struct DriverHeartbeat {
};

enum class IoDebugTestEnum : uint8_t {
  Two = 2,
  One = 1,
};

[[nodiscard]] constexpr std::expected<IoDebugTestEnum, CanError>
io_debug_test_enum_from_raw(uint8_t v) noexcept {
  switch (v) {
    case 2: return IoDebugTestEnum::Two;
    case 1: return IoDebugTestEnum::One;
    default: return std::unexpected(CanError::InvalidData);
  };
};

struct IoDebug {
};

struct MotorCmd {
};

struct MotorStatus {
};

struct SensorSonars {
};

