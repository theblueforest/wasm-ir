/*
 * wasm-ir - Intermediate Representation of WebAssembly
 * Copyright © 2019-2022 Blue Forest
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

// TODO: factorize
pub fn from_u32(value: u32) -> Vec<u8> {
  let mut result = Vec::new();
  let mut current = value;
  loop {
    let byte = (current & 0x7f) as u8;
    current >>= 7;
    if current == 0 {
      result.push(byte);
      break
    }
    result.push(byte + 0x80);
  }
  result
}

pub fn from_u64(value: u64) -> Vec<u8> {
  let mut result = Vec::new();
  let mut current = value;
  loop {
    let byte = (current & 0x7f) as u8;
    current >>= 7;
    if current == 0 {
      result.push(byte);
      break
    }
    result.push(byte + 0x80);
  }
  result
}
