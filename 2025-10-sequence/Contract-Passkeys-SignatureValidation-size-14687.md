
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { ISapientCompact } from "../../modules/interfaces/ISapient.sol";

import { LibBytes } from "../../utils/LibBytes.sol";
import { LibOptim } from "../../utils/LibOptim.sol";
import { WebAuthn } from "../../utils/WebAuthn.sol";

/// @title Passkeys
/// @author Agustin Aguilar, Michael Standen
/// @notice A sapient signer for passkeys
contract Passkeys is ISapientCompact {

  /// @notice Error thrown when the passkey signature is invalid
  error InvalidPasskeySignature(
    WebAuthn.WebAuthnAuth _webAuthnAuth, bool _requireUserVerification, bytes32 _x, bytes32 _y
  );

  function _rootForPasskey(
    bool _requireUserVerification,
    bytes32 _x,
    bytes32 _y,
    bytes32 _metadata
  ) internal pure returns (bytes32) {
    bytes32 a = LibOptim.fkeccak256(_x, _y);

    bytes32 ruv;
    assembly {
      ruv := _requireUserVerification
    }

    bytes32 b = LibOptim.fkeccak256(ruv, _metadata);
    return LibOptim.fkeccak256(a, b);
  }

  function _decodeSignature(
    bytes calldata _signature
  )
    internal
    pure
    returns (
      WebAuthn.WebAuthnAuth memory _webAuthnAuth,
      bool _requireUserVerification,
      bytes32 _x,
      bytes32 _y,
      bytes32 _metadata
    )
  {
    unchecked {
      // Global flag encoding:
      // 0000 000X : requireUserVerification
      // 0000 00X0 : 1 if 16 bits for authenticatorData size, 0 if 8 bits
      // 0000 0X00 : 1 if 16 bits for clientDataJSON size, 0 if 8 bits
      // 0000 X000 : 1 if 16 bits for challengeIndex, 0 if 8 bits
      // 000X 0000 : 1 if 16 bits for typeIndex, 0 if 8 bits
      // 00X0 0000 : 1 if fallback to abi decode data
      // 0X00 0000 : 1 if signature has metadata node
      // X000 0000 : unused

      bytes1 flags = _signature[0];
      if ((flags & 0x20) == 0) {
        _requireUserVerification = (flags & 0x01) != 0;
        uint256 bytesAuthDataSize = ((uint8(flags & 0x02)) >> 1) + 1;
        uint256 bytesClientDataJSONSize = ((uint8(flags & 0x04)) >> 2) + 1;
        uint256 bytesChallengeIndex = ((uint8(flags & 0x08)) >> 3) + 1;
        uint256 bytesTypeIndex = ((uint8(flags & 0x10)) >> 4) + 1;

        uint256 pointer = 1;

        if ((flags & 0x40) != 0) {
          (_metadata, pointer) = LibBytes.readBytes32(_signature, pointer);
        }

        {
          uint256 authDataSize;
          (authDataSize, pointer) = LibBytes.readUintX(_signature, pointer, bytesAuthDataSize);
          uint256 nextPointer = pointer + authDataSize;
          _webAuthnAuth.authenticatorData = _signature[pointer:nextPointer];
          pointer = nextPointer;
        }

        {
          uint256 clientDataJSONSize;
          (clientDataJSONSize, pointer) = LibBytes.readUintX(_signature, pointer, bytesClientDataJSONSize);
          uint256 nextPointer = pointer + clientDataJSONSize;
          _webAuthnAuth.clientDataJSON = string(_signature[pointer:nextPointer]);
          pointer = nextPointer;
        }

        (_webAuthnAuth.challengeIndex, pointer) = LibBytes.readUintX(_signature, pointer, bytesChallengeIndex);
        (_webAuthnAuth.typeIndex, pointer) = LibBytes.readUintX(_signature, pointer, bytesTypeIndex);

        (_webAuthnAuth.r, pointer) = LibBytes.readBytes32(_signature, pointer);
        (_webAuthnAuth.s, pointer) = LibBytes.readBytes32(_signature, pointer);

        (_x, pointer) = LibBytes.readBytes32(_signature, pointer);
        (_y, pointer) = LibBytes.readBytes32(_signature, pointer);
      } else {
        (_webAuthnAuth, _requireUserVerification, _x, _y, _metadata) =
          abi.decode(_signature[1:], (WebAuthn.WebAuthnAuth, bool, bytes32, bytes32, bytes32));
      }
    }
  }

  /// @inheritdoc ISapientCompact
  function recoverSapientSignatureCompact(bytes32 _digest, bytes calldata _signature) external view returns (bytes32) {
    (
      WebAuthn.WebAuthnAuth memory _webAuthnAuth,
      bool _requireUserVerification,
      bytes32 _x,
      bytes32 _y,
      bytes32 _metadata
    ) = _decodeSignature(_signature);

    if (!WebAuthn.verify(abi.encodePacked(_digest), _requireUserVerification, _webAuthnAuth, _x, _y)) {
      revert InvalidPasskeySignature(_webAuthnAuth, _requireUserVerification, _x, _y);
    }

    return _rootForPasskey(_requireUserVerification, _x, _y, _metadata);
  }

}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

/// @notice Library to encode strings in Base64.
/// @author Solady (https://github.com/vectorized/solady/blob/main/src/utils/Base64.sol)
/// @author Modified from Solmate (https://github.com/transmissions11/solmate/blob/main/src/utils/Base64.sol)
/// @author Modified from (https://github.com/Brechtpd/base64/blob/main/base64.sol) by Brecht Devos - <brecht@loopring.org>.
library Base64 {

  /// @dev Encodes `data` using the base64 encoding described in RFC 4648.
  /// See: https://datatracker.ietf.org/doc/html/rfc4648
  /// @param fileSafe  Whether to replace '+' with '-' and '/' with '_'.
  /// @param noPadding Whether to strip away the padding.
  function encode(bytes memory data, bool fileSafe, bool noPadding) internal pure returns (string memory result) {
    /// @solidity memory-safe-assembly
    assembly {
      let dataLength := mload(data)

      if dataLength {
        // Multiply by 4/3 rounded up.
        // The `shl(2, ...)` is equivalent to multiplying by 4.
        let encodedLength := shl(2, div(add(dataLength, 2), 3))

        // Set `result` to point to the start of the free memory.
        result := mload(0x40)

        // Store the table into the scratch space.
        // Offsetted by -1 byte so that the `mload` will load the character.
        // We will rewrite the free memory pointer at `0x40` later with
        // the allocated size.
        // The magic constant 0x0670 will turn "-_" into "+/".
        mstore(0x1f, "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef")
        mstore(0x3f, xor("ghijklmnopqrstuvwxyz0123456789-_", mul(iszero(fileSafe), 0x0670)))

        // Skip the first slot, which stores the length.
        let ptr := add(result, 0x20)
        let end := add(ptr, encodedLength)

        let dataEnd := add(add(0x20, data), dataLength)
        let dataEndValue := mload(dataEnd) // Cache the value at the `dataEnd` slot.
        mstore(dataEnd, 0x00) // Zeroize the `dataEnd` slot to clear dirty bits.

        // Run over the input, 3 bytes at a time.
        for { } 1 { } {
          data := add(data, 3) // Advance 3 bytes.
          let input := mload(data)

          // Write 4 bytes. Optimized for fewer stack operations.
          mstore8(0, mload(and(shr(18, input), 0x3F)))
          mstore8(1, mload(and(shr(12, input), 0x3F)))
          mstore8(2, mload(and(shr(6, input), 0x3F)))
          mstore8(3, mload(and(input, 0x3F)))
          mstore(ptr, mload(0x00))

          ptr := add(ptr, 4) // Advance 4 bytes.
          if iszero(lt(ptr, end)) { break }
        }
        mstore(dataEnd, dataEndValue) // Restore the cached value at `dataEnd`.
        mstore(0x40, add(end, 0x20)) // Allocate the memory.
        // Equivalent to `o = [0, 2, 1][dataLength % 3]`.
        let o := div(2, mod(dataLength, 3))
        // Offset `ptr` and pad with '='. We can simply write over the end.
        mstore(sub(ptr, o), shl(240, 0x3d3d))
        // Set `o` to zero if there is padding.
        o := mul(iszero(iszero(noPadding)), o)
        mstore(sub(ptr, o), 0) // Zeroize the slot after the string.
        mstore(result, sub(encodedLength, o)) // Store the length.
      }
    }
  }

  /// @dev Encodes `data` using the base64 encoding described in RFC 4648.
  /// Equivalent to `encode(data, false, false)`.
  function encode(
    bytes memory data
  ) internal pure returns (string memory result) {
    result = encode(data, false, false);
  }

  /// @dev Encodes `data` using the base64 encoding described in RFC 4648.
  /// Equivalent to `encode(data, fileSafe, false)`.
  function encode(bytes memory data, bool fileSafe) internal pure returns (string memory result) {
    result = encode(data, fileSafe, false);
  }

  /// @dev Decodes base64 encoded `data`.
  ///
  /// Supports:
  /// - RFC 4648 (both standard and file-safe mode).
  /// - RFC 3501 (63: ',').
  ///
  /// Does not support:
  /// - Line breaks.
  ///
  /// Note: For performance reasons,
  /// this function will NOT revert on invalid `data` inputs.
  /// Outputs for invalid inputs will simply be undefined behaviour.
  /// It is the user's responsibility to ensure that the `data`
  /// is a valid base64 encoded string.
  function decode(
    string memory data
  ) internal pure returns (bytes memory result) {
    /// @solidity memory-safe-assembly
    assembly {
      let dataLength := mload(data)

      if dataLength {
        let decodedLength := mul(shr(2, dataLength), 3)

        for { } 1 { } {
          // If padded.
          if iszero(and(dataLength, 3)) {
            let t := xor(mload(add(data, dataLength)), 0x3d3d)
            // forgefmt: disable-next-item
            decodedLength := sub(
                            decodedLength,
                            add(iszero(byte(30, t)), iszero(byte(31, t)))
                        )
            break
          }
          // If non-padded.
          decodedLength := add(decodedLength, sub(and(dataLength, 3), 1))
          break
        }
        result := mload(0x40)

        // Write the length of the bytes.
        mstore(result, decodedLength)

        // Skip the first slot, which stores the length.
        let ptr := add(result, 0x20)
        let end := add(ptr, decodedLength)

        // Load the table into the scratch space.
        // Constants are optimized for smaller bytecode with zero gas overhead.
        // `m` also doubles as the mask of the upper 6 bits.
        let m := 0xfc000000fc00686c7074787c8084888c9094989ca0a4a8acb0b4b8bcc0c4c8cc
        mstore(0x5b, m)
        mstore(0x3b, 0x04080c1014181c2024282c3034383c4044484c5054585c6064)
        mstore(0x1a, 0xf8fcf800fcd0d4d8dce0e4e8ecf0f4)

        for { } 1 { } {
          // Read 4 bytes.
          data := add(data, 4)
          let input := mload(data)

          // Write 3 bytes.
          // forgefmt: disable-next-item
          mstore(ptr, or(
                        and(m, mload(byte(28, input))),
                        shr(6, or(
                            and(m, mload(byte(29, input))),
                            shr(6, or(
                                and(m, mload(byte(30, input))),
                                shr(6, mload(byte(31, input)))
                            ))
                        ))
                    ))
          ptr := add(ptr, 3)
          if iszero(lt(ptr, end)) { break }
        }
        mstore(0x40, add(end, 0x20)) // Allocate the memory.
        mstore(end, 0) // Zeroize the slot after the bytes.
        mstore(0x60, 0) // Restore the zero slot.
      }
    }
  }

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.18;

/// @title Library for reading data from bytes arrays
/// @author Agustin Aguilar (aa@horizon.io), Michael Standen (mstan@horizon.io)
/// @notice This library contains functions for reading data from bytes arrays.
/// @dev These functions do not check if the input index is within the bounds of the data array.
/// @dev Reading out of bounds may return dirty values.
library LibBytes {

  function readFirstUint8(
    bytes calldata _data
  ) internal pure returns (uint8 a, uint256 newPointer) {
    assembly {
      let word := calldataload(_data.offset)
      a := shr(248, word)
      newPointer := 1
    }
  }

  function readUint8(bytes calldata _data, uint256 _index) internal pure returns (uint8 a, uint256 newPointer) {
    assembly {
      let word := calldataload(add(_index, _data.offset))
      a := shr(248, word)
      newPointer := add(_index, 1)
    }
  }

  function readUint16(bytes calldata _data, uint256 _index) internal pure returns (uint16 a, uint256 newPointer) {
    assembly {
      let word := calldataload(add(_index, _data.offset))
      a := shr(240, word)
      newPointer := add(_index, 2)
    }
  }

  function readUint24(bytes calldata _data, uint256 _index) internal pure returns (uint24 a, uint256 newPointer) {
    assembly {
      let word := calldataload(add(_index, _data.offset))
      a := shr(232, word)
      newPointer := add(_index, 3)
    }
  }

  function readUint64(bytes calldata _data, uint256 _index) internal pure returns (uint64 a, uint256 newPointer) {
    assembly {
      let word := calldataload(add(_index, _data.offset))
      a := shr(192, word)
      newPointer := add(_index, 8)
    }
  }

  function readUint160(bytes calldata _data, uint256 _index) internal pure returns (uint160 a, uint256 newPointer) {
    assembly {
      let word := calldataload(add(_index, _data.offset))
      a := shr(96, word)
      newPointer := add(_index, 20)
    }
  }

  function readUint256(bytes calldata _data, uint256 _index) internal pure returns (uint256 a, uint256 newPointer) {
    assembly {
      a := calldataload(add(_index, _data.offset))
      newPointer := add(_index, 32)
    }
  }

  function readUintX(
    bytes calldata _data,
    uint256 _index,
    uint256 _length
  ) internal pure returns (uint256 a, uint256 newPointer) {
    assembly {
      let word := calldataload(add(_index, _data.offset))
      let shift := sub(256, mul(_length, 8))
      a := and(shr(shift, word), sub(shl(mul(8, _length), 1), 1))
      newPointer := add(_index, _length)
    }
  }

  function readBytes4(bytes calldata _data, uint256 _pointer) internal pure returns (bytes4 a, uint256 newPointer) {
    assembly {
      let word := calldataload(add(_pointer, _data.offset))
      a := and(word, 0xffffffff00000000000000000000000000000000000000000000000000000000)
      newPointer := add(_pointer, 4)
    }
  }

  function readBytes32(bytes calldata _data, uint256 _pointer) internal pure returns (bytes32 a, uint256 newPointer) {
    assembly {
      a := calldataload(add(_pointer, _data.offset))
      newPointer := add(_pointer, 32)
    }
  }

  function readAddress(bytes calldata _data, uint256 _index) internal pure returns (address a, uint256 newPointer) {
    assembly {
      let word := calldataload(add(_index, _data.offset))
      a := and(shr(96, word), 0xffffffffffffffffffffffffffffffffffffffff)
      newPointer := add(_index, 20)
    }
  }

  /// @dev ERC-2098 Compact Signature
  function readRSVCompact(
    bytes calldata _data,
    uint256 _index
  ) internal pure returns (bytes32 r, bytes32 s, uint8 v, uint256 newPointer) {
    uint256 yParityAndS;
    assembly {
      r := calldataload(add(_index, _data.offset))
      yParityAndS := calldataload(add(_index, add(_data.offset, 32)))
      newPointer := add(_index, 64)
    }
    uint256 yParity = uint256(yParityAndS >> 255);
    s = bytes32(uint256(yParityAndS) & ((1 << 255) - 1));
    v = uint8(yParity) + 27;
  }

}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

import { Base64 } from "./Base64.sol";
import { P256 } from "./P256.sol";

/// @notice WebAuthn helper.
/// @author Solady (https://github.com/vectorized/solady/blob/main/src/utils/WebAuthn.sol)
/// @author Modified from Daimo WebAuthn (https://github.com/daimo-eth/p256-verifier/blob/master/src/WebAuthn.sol)
/// @author Modified from Coinbase WebAuthn (https://github.com/base-org/webauthn-sol/blob/main/src/WebAuthn.sol)
library WebAuthn {

  /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
  /*                          STRUCTS                           */
  /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

  /// @dev Helps make encoding and decoding easier, alleviates stack-too-deep.
  struct WebAuthnAuth {
    // The WebAuthn authenticator data.
    // See: https://www.w3.org/TR/webauthn-2/#dom-authenticatorassertionresponse-authenticatordata.
    bytes authenticatorData;
    // The WebAuthn client data JSON.
    // See: https://www.w3.org/TR/webauthn-2/#dom-authenticatorresponse-clientdatajson.
    string clientDataJSON;
    // Start index of "challenge":"..." in `clientDataJSON`.
    uint256 challengeIndex;
    // Start index of "type":"..." in `clientDataJSON`.
    uint256 typeIndex;
    // The r value of secp256r1 signature.
    bytes32 r;
    // The s value of secp256r1 signature.
    bytes32 s;
  }

  /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
  /*              WEBAUTHN VERIFICATION OPERATIONS              */
  /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

  /// @dev Verifies a Webauthn Authentication Assertion.
  /// See: https://www.w3.org/TR/webauthn-2/#sctn-verifying-assertion.
  ///
  /// We do not verify all the steps as described in the specification, only ones
  /// relevant to our context. Please carefully read through this list before usage.
  ///
  /// Specifically, we do verify the following:
  /// - Verify that `authenticatorData` (which comes from the authenticator,
  ///   such as iCloud Keychain) indicates a well-formed assertion with the
  ///   "User Present" bit set. If `requireUserVerification` is set, checks that the
  ///   authenticator enforced user verification. User verification should be required
  ///   if, and only if, `options.userVerification` is set to required in the request.
  /// - Verifies that the client JSON is of type "webauthn.get",
  ///   i.e. the client was responding to a request to assert authentication.
  /// - Verifies that the client JSON contains the requested challenge.
  /// - Verifies that (r, s) constitute a valid signature over both the
  ///   `authData` and client JSON, for public key (x, y).
  ///
  /// We make some assumptions about the particular use case of this verifier,
  /// so we do NOT verify the following:
  /// - Does NOT verify that the origin in the `clientDataJSON` matches the
  ///   Relying Party's origin: it is considered the authenticator's responsibility to
  ///   ensure that the user is interacting with the correct RP. This is enforced by
  ///   most high quality authenticators properly, particularly the iCloud Keychain
  ///   and Google Password Manager were tested.
  /// - Does NOT verify That `topOrigin` in `clientDataJSON` is well-formed:
  ///   We assume it would never be present, i.e. the credentials are never used in a
  ///   cross-origin/iframe context. The website/app set up should disallow cross-origin
  ///   usage of the credentials. This is the default behavior for created credentials
  ///   in common settings.
  /// - Does NOT verify that the `rpIdHash` in `authenticatorData` is the SHA-256 hash
  ///   of the RP ID expected by the Relying Party:
  ///   this means that we rely on the authenticator to properly enforce
  ///   credentials to be used only by the correct RP.
  ///   This is generally enforced with features like Apple App Site Association
  ///   and Google Asset Links. To protect from edge cases in which a previously-linked
  ///   RP ID is removed from the authorized RP IDs, we recommend that messages
  ///   signed by the authenticator include some expiry mechanism.
  /// - Does NOT verify the credential backup state: this assumes the credential backup
  ///   state is NOT used as part of Relying Party business logic or policy.
  /// - Does NOT verify the values of the client extension outputs:
  ///   this assumes that the Relying Party does not use client extension outputs.
  /// - Does NOT verify the signature counter: signature counters are intended to enable
  ///   risk scoring for the Relying Party. This assumes risk scoring is not used as part
  ///   of Relying Party business logic or policy.
  /// - Does NOT verify the attestation object: this assumes that
  ///   response.attestationObject is NOT present in the response,
  ///   i.e. the RP does not intend to verify an attestation.
  function verify(
    bytes memory challenge,
    bool requireUserVerification,
    WebAuthnAuth memory auth,
    bytes32 x,
    bytes32 y
  ) internal view returns (bool result) {
    bytes32 messageHash;
    string memory encoded = Base64.encode(challenge, true, true);
    /// @solidity memory-safe-assembly
    assembly {
      let clientDataJSON := mload(add(auth, 0x20))
      let n := mload(clientDataJSON) // `clientDataJSON`'s length.
      let o := add(clientDataJSON, 0x20) // Start of `clientData`'s bytes.
      {
        let c := mload(add(auth, 0x40)) // Challenge index in `clientDataJSON`.
        let t := mload(add(auth, 0x60)) // Type index in `clientDataJSON`.
        let l := mload(encoded) // Cache `encoded`'s length.
        let q := add(l, 0x0d) // Length of `encoded` prefixed with '"challenge":"'.
        mstore(encoded, shr(152, '"challenge":"')) // Temp prefix with '"challenge":"'.
        result :=
          and(
            // 11. Verify JSON's type. Also checks for possible addition overflows.
            and(
              eq(shr(88, mload(add(o, t))), shr(88, '"type":"webauthn.get"')), lt(shr(128, or(t, c)), lt(add(0x14, t), n))
            ),
            // 12. Verify JSON's challenge. Includes a check for the closing '"'.
            and(
              eq(keccak256(add(o, c), q), keccak256(add(encoded, 0x13), q)),
              and(eq(byte(0, mload(add(add(o, c), q))), 34), lt(add(q, c), n))
            )
          )
        mstore(encoded, l) // Restore `encoded`'s length, in case of string interning.
      }
      // Skip 13., 14., 15.
      let l := mload(mload(auth)) // Length of `authenticatorData`.
      // 16. Verify that the "User Present" flag is set (bit 0).
      // 17. Verify that the "User Verified" flag is set (bit 2), if required.
      // See: https://www.w3.org/TR/webauthn-2/#flags.
      let u := or(1, shl(2, iszero(iszero(requireUserVerification))))
      result := and(and(result, gt(l, 0x20)), eq(and(mload(add(mload(auth), 0x21)), u), u))
      if result {
        let p := add(mload(auth), 0x20) // Start of `authenticatorData`'s bytes.
        let e := add(p, l) // Location of the word after `authenticatorData`.
        let w := mload(e) // Cache the word after `authenticatorData`.
        // 19. Compute `sha256(clientDataJSON)`.
        // 20. Compute `sha256(authenticatorData ‖ sha256(clientDataJSON))`.
        // forgefmt: disable-next-item
        messageHash := mload(staticcall(gas(),
                    shl(1, staticcall(gas(), 2, o, n, e, 0x20)), p, add(l, 0x20), 0x01, 0x20))
        mstore(e, w) // Restore the word after `authenticatorData`, in case of reuse.
        // `returndatasize()` is `0x20` on `sha256` success, and `0x00` otherwise.
        if iszero(returndatasize()) { invalid() }
      }
    }
    // `P256.verifySignature` returns false if `s > N/2` due to the malleability check.
    if (result) {
      result = P256.verifySignature(messageHash, auth.r, auth.s, x, y);
    }
  }

  /// @dev Plain variant of verify.
  function verify(
    bytes memory challenge,
    bool requireUserVerification,
    bytes memory authenticatorData,
    string memory clientDataJSON,
    uint256 challengeIndex,
    uint256 typeIndex,
    bytes32 r,
    bytes32 s,
    bytes32 x,
    bytes32 y
  ) internal view returns (bool) {
    return verify(
      challenge,
      requireUserVerification,
      WebAuthnAuth(authenticatorData, clientDataJSON, challengeIndex, typeIndex, r, s),
      x,
      y
    );
  }

  /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
  /*                ENCODING / DECODING HELPERS                 */
  /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

  /// @dev Returns `abi.encode(auth)`.
  function encodeAuth(
    WebAuthnAuth memory auth
  ) internal pure returns (bytes memory) {
    return abi.encode(auth);
  }

  /// @dev Performs a best-effort attempt to `abi.decode(auth)`. Won't revert.
  /// If any fields cannot be successfully extracted, `decoded` will not be populated,
  /// which will cause `verify` to return false (as `clientDataJSON` is empty).
  function tryDecodeAuth(
    bytes memory encodedAuth
  ) internal pure returns (WebAuthnAuth memory decoded) {
    /// @solidity memory-safe-assembly
    assembly {
      for { let n := mload(encodedAuth) } iszero(lt(n, 0xc0)) { } {
        let o := add(encodedAuth, 0x20) // Start of `encodedAuth`'s bytes.
        let e := add(o, n) // End of `encodedAuth` in memory.
        let p := add(mload(o), o) // Start of `encodedAuth`.
        if or(gt(add(p, 0xc0), e), lt(p, o)) { break }
        let authenticatorData := add(mload(p), p)
        let clientDataJSON := add(mload(add(p, 0x20)), p)
        if or(or(gt(authenticatorData, e), lt(authenticatorData, p)), or(gt(clientDataJSON, e), lt(clientDataJSON, p)))
        {
          break
        }
        if or(
          gt(add(add(authenticatorData, 0x20), mload(authenticatorData)), e),
          gt(add(add(clientDataJSON, 0x20), mload(clientDataJSON)), e)
        ) { break }
        mstore(decoded, authenticatorData) // `authenticatorData`.
        mstore(add(decoded, 0x20), clientDataJSON) // `clientDataJSON`.
        mstore(add(decoded, 0x40), mload(add(p, 0x40))) // `challengeIndex`.
        mstore(add(decoded, 0x60), mload(add(p, 0x60))) // `typeIndex`.
        mstore(add(decoded, 0x80), mload(add(p, 0x80))) // `r`.
        mstore(add(decoded, 0xa0), mload(add(p, 0xa0))) // `s`.
        break
      }
    }
  }

  /// @dev Returns the compact encoding of `auth`:
  /// ```
  ///     abi.encodePacked(
  ///         uint16(auth.authenticatorData.length),
  ///         bytes(auth.authenticatorData),
  ///         bytes(auth.clientDataJSON),
  ///         uint16(auth.challengeIndex),
  ///         uint16(auth.typeIndex),
  ///         bytes32(auth.r),
  ///         bytes32(auth.s)
  ///     )
  /// ```
  /// Returns the empty string if any length or index exceeds 16 bits.
  function tryEncodeAuthCompact(
    WebAuthnAuth memory auth
  ) internal pure returns (bytes memory result) {
    /// @solidity memory-safe-assembly
    assembly {
      function copyBytes(o_, s_, c_) -> _e {
        mstore(o_, shl(240, mload(s_)))
        o_ := add(o_, c_)
        _e := add(o_, mload(s_)) // The end of the bytes.
        for { let d_ := sub(add(0x20, s_), o_) } 1 { } {
          mstore(o_, mload(add(d_, o_)))
          o_ := add(o_, 0x20)
          if iszero(lt(o_, _e)) { break }
        }
      }
      let clientDataJSON := mload(add(0x20, auth))
      let c := mload(add(0x40, auth)) // `challengeIndex`.
      let t := mload(add(0x60, auth)) // `typeIndex`.
      // If none of the lengths are more than `0xffff`.
      if iszero(shr(16, or(or(t, c), or(mload(mload(auth)), mload(clientDataJSON))))) {
        result := mload(0x40)
        // `authenticatorData`, `clientDataJSON`.
        let o := copyBytes(copyBytes(add(result, 0x20), mload(auth), 2), clientDataJSON, 0)
        mstore(o, or(shl(240, c), shl(224, t))) // `challengeIndex`, `typeIndex`.
        mstore(add(o, 0x04), mload(add(0x80, auth))) // `r`.
        mstore(add(o, 0x24), mload(add(0xa0, auth))) // `s`.
        mstore(result, sub(add(o, 0x24), result)) // Store the length.
        mstore(add(o, 0x44), 0) // Zeroize the slot after the string.
        mstore(0x40, add(o, 0x64)) // Allocate memory .
      }
    }
  }

  /// @dev Approximately the same gas as `tryDecodeAuth`, but helps save on calldata.
  /// If any fields cannot be successfully extracted, `decoded` will not be populated,
  /// which will cause `verify` to return false (as `clientDataJSON` is empty).
  function tryDecodeAuthCompact(
    bytes memory encodedAuth
  ) internal pure returns (WebAuthnAuth memory decoded) {
    /// @solidity memory-safe-assembly
    assembly {
      function extractBytes(o_, l_) -> _m {
        _m := mload(0x40) // Grab the free memory pointer.
        let s_ := add(_m, 0x20)
        for { let i_ := 0 } 1 { } {
          mstore(add(s_, i_), mload(add(o_, i_)))
          i_ := add(i_, 0x20)
          if iszero(lt(i_, l_)) { break }
        }
        mstore(_m, l_) // Store the length.
        mstore(add(l_, s_), 0) // Zeroize the slot after the string.
        mstore(0x40, add(0x20, add(l_, s_))) // Allocate memory.
      }
      let n := mload(encodedAuth)
      if iszero(lt(n, 0x46)) {
        let o := add(encodedAuth, 0x20) // Start of `encodedAuth`'s bytes.
        let e := add(o, n) // End of `encodedAuth` in memory.
        n := shr(240, mload(o)) // Length of `authenticatorData`.
        let a := add(o, 0x02) // Start of `authenticatorData`.
        let c := add(a, n) // Start of `clientDataJSON`.
        let j := sub(e, 0x44) // Start of `challengeIndex`.
        if iszero(gt(c, j)) {
          mstore(decoded, extractBytes(a, n)) // `authenticatorData`.
          mstore(add(decoded, 0x20), extractBytes(c, sub(j, c))) // `clientDataJSON`.
          mstore(add(decoded, 0x40), shr(240, mload(j))) // `challengeIndex`.
          mstore(add(decoded, 0x60), shr(240, mload(add(j, 0x02)))) // `typeIndex`.
          mstore(add(decoded, 0x80), mload(add(j, 0x04))) // `r`.
          mstore(add(decoded, 0xa0), mload(add(j, 0x24))) // `s`.
        }
      }
    }
  }

  /// @dev Calldata variant of `tryDecodeAuthCompact`.
  function tryDecodeAuthCompactCalldata(
    bytes calldata encodedAuth
  ) internal pure returns (WebAuthnAuth memory decoded) {
    /// @solidity memory-safe-assembly
    assembly {
      function extractBytes(o_, l_) -> _m {
        _m := mload(0x40) // Grab the free memory pointer.
        let s_ := add(_m, 0x20)
        calldatacopy(s_, o_, l_)
        mstore(_m, l_) // Store the length.
        mstore(add(l_, s_), 0) // Zeroize the slot after the string.
        mstore(0x40, add(0x20, add(l_, s_))) // Allocate memory.
      }
      if iszero(lt(encodedAuth.length, 0x46)) {
        let e := add(encodedAuth.offset, encodedAuth.length) // End of `encodedAuth`.
        let n := shr(240, calldataload(encodedAuth.offset)) // Length of `authenticatorData`.
        let a := add(encodedAuth.offset, 0x02) // Start of `authenticatorData`.
        let c := add(a, n) // Start of `clientDataJSON`.
        let j := sub(e, 0x44) // Start of `challengeIndex`.
        if iszero(gt(c, j)) {
          mstore(decoded, extractBytes(a, n)) // `authenticatorData`.
          mstore(add(decoded, 0x20), extractBytes(c, sub(j, c))) // `clientDataJSON`.
          mstore(add(decoded, 0x40), shr(240, calldataload(j))) // `challengeIndex`.
          mstore(add(decoded, 0x60), shr(240, calldataload(add(j, 0x02)))) // `typeIndex`.
          mstore(add(decoded, 0x80), calldataload(add(j, 0x04))) // `r`.
          mstore(add(decoded, 0xa0), calldataload(add(j, 0x24))) // `s`.
        }
      }
    }
  }

}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

/// @notice Gas optimized P256 wrapper.
/// @author Solady (https://github.com/vectorized/solady/blob/main/src/utils/P256.sol)
/// @author Modified from Daimo P256 Verifier (https://github.com/daimo-eth/p256-verifier/blob/master/src/P256.sol)
/// @author Modified from OpenZeppelin (https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/cryptography/P256.sol)
library P256 {

  /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
  /*                        CUSTOM ERRORS                       */
  /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

  /// @dev Unable to verify the P256 signature, due to missing
  /// RIP-7212 P256 verifier precompile and missing Solidity P256 verifier.
  error P256VerificationFailed();

  /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
  /*                         CONSTANTS                          */
  /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

  /// @dev Address of the Solidity P256 verifier.
  /// Please make sure the contract is deployed onto the chain you are working on.
  /// See: https://gist.github.com/Vectorized/599b0d8a94d21bc74700eb1354e2f55c
  /// Unlike RIP-7212, this verifier returns `uint256(0)` on failure, to
  /// facilitate easier existence check. This verifier will also never revert.
  address internal constant VERIFIER = 0x000000000000D01eA45F9eFD5c54f037Fa57Ea1a;

  /// @dev Address of the RIP-7212 P256 verifier precompile.
  /// Currently, we don't support EIP-7212's precompile at 0x0b as it has not been finalized.
  /// See: https://github.com/ethereum/RIPs/blob/master/RIPS/rip-7212.md
  address internal constant RIP_PRECOMPILE = 0x0000000000000000000000000000000000000100;

  /// @dev The order of the secp256r1 elliptic curve.
  uint256 internal constant N = 0xFFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551;

  /// @dev `N/2`. Used for checking the malleability of the signature.
  uint256 private constant _HALF_N = 0x7fffffff800000007fffffffffffffffde737d56d38bcf4279dce5617e3192a8;

  /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
  /*                P256 VERIFICATION OPERATIONS                */
  /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

  /// @dev Returns if the signature (`r`, `s`) is valid for `hash` and public key (`x`, `y`).
  /// Does NOT include the malleability check.
  function verifySignatureAllowMalleability(
    bytes32 hash,
    bytes32 r,
    bytes32 s,
    bytes32 x,
    bytes32 y
  ) internal view returns (bool isValid) {
    /// @solidity memory-safe-assembly
    assembly {
      let m := mload(0x40)
      mstore(m, hash)
      mstore(add(m, 0x20), r)
      mstore(add(m, 0x40), s)
      mstore(add(m, 0x60), x)
      mstore(add(m, 0x80), y)
      mstore(0x00, 0) // Zeroize the return slot before the staticcalls.
      pop(staticcall(gas(), RIP_PRECOMPILE, m, 0xa0, 0x00, 0x20))
      // RIP-7212 dictates that success returns `uint256(1)`.
      // But failure returns zero returndata, which is ambiguous.
      if iszero(returndatasize()) {
        pop(staticcall(gas(), VERIFIER, m, 0xa0, returndatasize(), 0x20))
        // Unlike RIP-7212, the verifier returns `uint256(0)` on failure,
        // allowing us to use the returndatasize to determine existence.
        if iszero(returndatasize()) {
          mstore(returndatasize(), 0xd0d5039b) // `P256VerificationFailed()`.
          revert(0x1c, 0x04)
        }
      }
      isValid := eq(1, mload(0x00))
    }
  }

  /// @dev Returns if the signature (`r`, `s`) is valid for `hash` and public key (`x`, `y`).
  /// Includes the malleability check.
  function verifySignature(
    bytes32 hash,
    bytes32 r,
    bytes32 s,
    bytes32 x,
    bytes32 y
  ) internal view returns (bool isValid) {
    /// @solidity memory-safe-assembly
    assembly {
      let m := mload(0x40)
      mstore(m, hash)
      mstore(add(m, 0x20), r)
      mstore(add(m, 0x40), s)
      mstore(add(m, 0x60), x)
      mstore(add(m, 0x80), y)
      mstore(0x00, 0) // Zeroize the return slot before the staticcalls.
      pop(staticcall(gas(), RIP_PRECOMPILE, m, 0xa0, 0x00, 0x20))
      // RIP-7212 dictates that success returns `uint256(1)`.
      // But failure returns zero returndata, which is ambiguous.
      if iszero(returndatasize()) {
        pop(staticcall(gas(), VERIFIER, m, 0xa0, returndatasize(), 0x20))
        // Unlike RIP-7212, the verifier returns `uint256(0)` on failure,
        // allowing us to use the returndatasize to determine existence.
        if iszero(returndatasize()) {
          mstore(returndatasize(), 0xd0d5039b) // `P256VerificationFailed()`.
          revert(0x1c, 0x04)
        }
      }
      // Optimize for happy path. Users are unlikely to pass in malleable signatures.
      isValid := lt(gt(s, _HALF_N), eq(1, mload(0x00)))
    }
  }

  /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
  /*                      OTHER OPERATIONS                      */
  /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

  /// @dev Returns `s` normalized to the lower half of the curve.
  function normalized(
    bytes32 s
  ) internal pure returns (bytes32 result) {
    /// @solidity memory-safe-assembly
    assembly {
      result := xor(s, mul(xor(sub(N, s), s), gt(s, _HALF_N)))
    }
  }

  /// @dev Helper function for `abi.decode(encoded, (bytes32, bytes32))`.
  /// If `encoded.length < 64`, `(x, y)` will be `(0, 0)`, which is an invalid point.
  function tryDecodePoint(
    bytes memory encoded
  ) internal pure returns (bytes32 x, bytes32 y) {
    /// @solidity memory-safe-assembly
    assembly {
      let t := gt(mload(encoded), 0x3f)
      x := mul(mload(add(encoded, 0x20)), t)
      y := mul(mload(add(encoded, 0x40)), t)
    }
  }

  /// @dev Helper function for `abi.decode(encoded, (bytes32, bytes32))`.
  /// If `encoded.length < 64`, `(x, y)` will be `(0, 0)`, which is an invalid point.
  function tryDecodePointCalldata(
    bytes calldata encoded
  ) internal pure returns (bytes32 x, bytes32 y) {
    /// @solidity memory-safe-assembly
    assembly {
      let t := gt(encoded.length, 0x3f)
      x := mul(calldataload(encoded.offset), t)
      y := mul(calldataload(add(encoded.offset, 0x20)), t)
    }
  }

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.18;

/// @title LibOptim
/// @author Agustin Aguilar
/// @notice Library for optimized EVM operations
library LibOptim {

  /**
   * @notice Computes the keccak256 hash of two 32-byte inputs.
   * @dev It uses only scratch memory space.
   * @param _a The first 32 bytes of the hash.
   * @param _b The second 32 bytes of the hash.
   * @return c The keccak256 hash of the two 32-byte inputs.
   */
  function fkeccak256(bytes32 _a, bytes32 _b) internal pure returns (bytes32 c) {
    assembly {
      mstore(0, _a)
      mstore(32, _b)
      c := keccak256(0, 64)
    }
  }

  /**
   * @notice Returns the return data from the last call.
   * @return r The return data from the last call.
   */
  function returnData() internal pure returns (bytes memory r) {
    assembly {
      let size := returndatasize()
      r := mload(0x40)
      let start := add(r, 32)
      mstore(0x40, add(start, size))
      mstore(r, size)
      returndatacopy(start, 0, size)
    }
  }

  /**
   * @notice Calls another contract with the given parameters.
   * @dev This method doesn't increase the memory pointer.
   * @param _to The address of the contract to call.
   * @param _val The value to send to the contract.
   * @param _gas The amount of gas to provide for the call.
   * @param _data The data to send to the contract.
   * @return r The success status of the call.
   */
  function call(address _to, uint256 _val, uint256 _gas, bytes memory _data) internal returns (bool r) {
    assembly {
      r := call(_gas, _to, _val, add(_data, 32), mload(_data), 0, 0)
    }
  }

  /**
   * @notice Calls another contract with the given parameters, using delegatecall.
   * @dev This method doesn't increase the memory pointer.
   * @param _to The address of the contract to call.
   * @param _gas The amount of gas to provide for the call.
   * @param _data The data to send to the contract.
   * @return r The success status of the call.
   */
  function delegatecall(address _to, uint256 _gas, bytes memory _data) internal returns (bool r) {
    assembly {
      r := delegatecall(_gas, _to, add(_data, 32), mload(_data), 0, 0)
    }
  }

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Payload } from "../Payload.sol";

/// @title ISapient
/// @author Agustin Aguilar, Michael Standen
/// @notice Sapient signers take an explicit payload and return their own "imageHash" as result
/// @dev The consumer of this signer must validate if the imageHash is valid or not, for the desired configuration
interface ISapient {

  /// @notice Recovers the image hash of a given signature
  /// @param payload The payload to recover the signature from
  /// @param signature The signature to recover the image hash from
  /// @return imageHash The recovered image hash
  function recoverSapientSignature(
    Payload.Decoded calldata payload,
    bytes calldata signature
  ) external view returns (bytes32 imageHash);

}

/// @title ISapientCompact
/// @author Agustin Aguilar, Michael Standen
/// @notice Sapient signers take a compacted payload and return their own "imageHash" as result
/// @dev The consumer of this signer must validate if the imageHash is valid or not, for the desired configuration
interface ISapientCompact {

  /// @notice Recovers the image hash of a given signature, using a hashed payload
  /// @param digest The digest of the payload
  /// @param signature The signature to recover the image hash from
  /// @return imageHash The recovered image hash
  function recoverSapientSignatureCompact(
    bytes32 digest,
    bytes calldata signature
  ) external view returns (bytes32 imageHash);

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { LibBytes } from "../utils/LibBytes.sol";

using LibBytes for bytes;

/// @title Payload
/// @author Agustin Aguilar, Michael Standen, William Hua
/// @notice Library for encoding and decoding payloads
library Payload {

  /// @notice Error thrown when the kind is invalid
  error InvalidKind(uint8 kind);

  /// @dev keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)")
  bytes32 private constant EIP712_DOMAIN_TYPEHASH = 0x8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f;

  /// @dev keccak256("Sequence Wallet")
  bytes32 private constant EIP712_DOMAIN_NAME_SEQUENCE =
    0x4aa45ca7ad825ceb1bf35643f0a58c295239df563b1b565c2485f96477c56318;

  /// @dev keccak256("3")
  bytes32 private constant EIP712_DOMAIN_VERSION_SEQUENCE =
    0x2a80e1ef1d7842f27f2e6be0972bb708b9a135c38860dbe73c27c3486c34f4de;

  function domainSeparator(bool _noChainId, address _wallet) internal view returns (bytes32 _domainSeparator) {
    return keccak256(
      abi.encode(
        EIP712_DOMAIN_TYPEHASH,
        EIP712_DOMAIN_NAME_SEQUENCE,
        EIP712_DOMAIN_VERSION_SEQUENCE,
        _noChainId ? uint256(0) : uint256(block.chainid),
        _wallet
      )
    );
  }

  /// @dev keccak256("Call(address to,uint256 value,bytes data,uint256 gasLimit,bool delegateCall,bool onlyFallback,uint256 behaviorOnError)")
  bytes32 private constant CALL_TYPEHASH = 0x0603985259a953da1f65a522f589c17bd1d0117ec1d3abb7c0788aef251ef437;

  /// @dev keccak256("Calls(Call[] calls,uint256 space,uint256 nonce,address[] wallets)Call(address to,uint256 value,bytes data,uint256 gasLimit,bool delegateCall,bool onlyFallback,uint256 behaviorOnError)")
  bytes32 private constant CALLS_TYPEHASH = 0x11e1e4079a79a66e4ade50033cfe2678cdd5341d2dfe5ef9513edb1a0be147a2;

  /// @dev keccak256("Message(bytes message,address[] wallets)")
  bytes32 private constant MESSAGE_TYPEHASH = 0xe19a3b94fc3c7ece3f890d98a99bc422615537a08dea0603fa8425867d87d466;

  /// @dev keccak256("ConfigUpdate(bytes32 imageHash,address[] wallets)")
  bytes32 private constant CONFIG_UPDATE_TYPEHASH = 0x11fdeb7e8373a1aa96bfac8d0ea91526b2c5d15e5cee20e0543e780258f3e8e4;

  /// @notice Kind of transaction
  uint8 public constant KIND_TRANSACTIONS = 0x00;
  /// @notice Kind of digest
  uint8 public constant KIND_MESSAGE = 0x01;
  /// @notice Kind of config update
  uint8 public constant KIND_CONFIG_UPDATE = 0x02;
  /// @notice Kind of message
  uint8 public constant KIND_DIGEST = 0x03;

  /// @notice Behavior on error: ignore error
  uint8 public constant BEHAVIOR_IGNORE_ERROR = 0x00;
  /// @notice Behavior on error: revert on error
  uint8 public constant BEHAVIOR_REVERT_ON_ERROR = 0x01;
  /// @notice Behavior on error: abort on error
  uint8 public constant BEHAVIOR_ABORT_ON_ERROR = 0x02;

  /// @notice Payload call information
  /// @param to Address of the target contract
  /// @param value Value to send with the call
  /// @param data Data to send with the call
  /// @param gasLimit Gas limit for the call
  /// @param delegateCall If the call is a delegate call
  /// @param onlyFallback If the call should only be executed in an error scenario
  /// @param behaviorOnError Behavior on error
  struct Call {
    address to;
    uint256 value;
    bytes data;
    uint256 gasLimit;
    bool delegateCall;
    bool onlyFallback;
    uint256 behaviorOnError;
  }

  /// @notice Decoded payload
  /// @param kind Kind of payload
  /// @param noChainId If the chain ID should be omitted
  /// @param calls Array of calls (transaction kind)
  /// @param space Nonce space for the calls (transaction kind)
  /// @param nonce Nonce value for the calls (transaction kind)
  /// @param message Message to validate (message kind)
  /// @param imageHash Image hash to update to (config update kind)
  /// @param digest Digest to validate (digest kind)
  /// @param parentWallets Parent wallets
  struct Decoded {
    uint8 kind;
    bool noChainId;
    // Transaction kind
    Call[] calls;
    uint256 space;
    uint256 nonce;
    // Message kind
    bytes message;
    // Config update kind
    bytes32 imageHash;
    // Digest kind for 1271
    bytes32 digest;
    // Parent wallets
    address[] parentWallets;
  }

  function fromMessage(
    bytes memory message
  ) internal pure returns (Decoded memory _decoded) {
    _decoded.kind = KIND_MESSAGE;
    _decoded.message = message;
  }

  function fromConfigUpdate(
    bytes32 imageHash
  ) internal pure returns (Decoded memory _decoded) {
    _decoded.kind = KIND_CONFIG_UPDATE;
    _decoded.imageHash = imageHash;
  }

  function fromDigest(
    bytes32 digest
  ) internal pure returns (Decoded memory _decoded) {
    _decoded.kind = KIND_DIGEST;
    _decoded.digest = digest;
  }

  function fromPackedCalls(
    bytes calldata packed
  ) internal view returns (Decoded memory _decoded) {
    _decoded.kind = KIND_TRANSACTIONS;

    // Read the global flag
    (uint256 globalFlag, uint256 pointer) = packed.readFirstUint8();

    // First bit determines if space is zero or not
    if (globalFlag & 0x01 == 0x01) {
      _decoded.space = 0;
    } else {
      (_decoded.space, pointer) = packed.readUint160(pointer);
    }

    // Next 3 bits determine the size of the nonce
    uint256 nonceSize = (globalFlag >> 1) & 0x07;

    if (nonceSize > 0) {
      // Read the nonce
      (_decoded.nonce, pointer) = packed.readUintX(pointer, nonceSize);
    }

    uint256 numCalls;

    // Bit 5 determines if the batch contains a single call
    if (globalFlag & 0x10 == 0x10) {
      numCalls = 1;
    } else {
      // Bit 6 determines if the number of calls uses 1 byte or 2 bytes
      if (globalFlag & 0x20 == 0x20) {
        (numCalls, pointer) = packed.readUint16(pointer);
      } else {
        (numCalls, pointer) = packed.readUint8(pointer);
      }
    }

    // Read the calls
    _decoded.calls = new Call[](numCalls);

    for (uint256 i = 0; i < numCalls; i++) {
      uint8 flags;
      (flags, pointer) = packed.readUint8(pointer);

      // First bit determines if this is a call to self
      // or a call to another address
      if (flags & 0x01 == 0x01) {
        // Call to self
        _decoded.calls[i].to = address(this);
      } else {
        // Call to another address
        (_decoded.calls[i].to, pointer) = packed.readAddress(pointer);
      }

      // Second bit determines if the call has value or not
      if (flags & 0x02 == 0x02) {
        (_decoded.calls[i].value, pointer) = packed.readUint256(pointer);
      }

      // Third bit determines if the call has data or not
      if (flags & 0x04 == 0x04) {
        // 3 bytes determine the size of the calldata
        uint256 calldataSize;
        (calldataSize, pointer) = packed.readUint24(pointer);
        _decoded.calls[i].data = packed[pointer:pointer + calldataSize];
        pointer += calldataSize;
      }

      // Fourth bit determines if the call has a gas limit or not
      if (flags & 0x08 == 0x08) {
        (_decoded.calls[i].gasLimit, pointer) = packed.readUint256(pointer);
      }

      // Fifth bit determines if the call is a delegate call or not
      _decoded.calls[i].delegateCall = (flags & 0x10 == 0x10);

      // Sixth bit determines if the call is fallback only
      _decoded.calls[i].onlyFallback = (flags & 0x20 == 0x20);

      // Last 2 bits are directly mapped to the behavior on error
      _decoded.calls[i].behaviorOnError = (flags & 0xC0) >> 6;
    }
  }

  function hashCall(
    Call memory c
  ) internal pure returns (bytes32) {
    return keccak256(
      abi.encode(
        CALL_TYPEHASH, c.to, c.value, keccak256(c.data), c.gasLimit, c.delegateCall, c.onlyFallback, c.behaviorOnError
      )
    );
  }

  function hashCalls(
    Call[] memory calls
  ) internal pure returns (bytes32) {
    // In EIP712, an array is often hashed as the keccak256 of the concatenated
    // hashes of each item. So we hash each Call, pack them, and hash again.
    bytes32[] memory callHashes = new bytes32[](calls.length);
    for (uint256 i = 0; i < calls.length; i++) {
      callHashes[i] = hashCall(calls[i]);
    }
    return keccak256(abi.encodePacked(callHashes));
  }

  function toEIP712(
    Decoded memory _decoded
  ) internal pure returns (bytes32) {
    bytes32 walletsHash = keccak256(abi.encodePacked(_decoded.parentWallets));

    if (_decoded.kind == KIND_TRANSACTIONS) {
      bytes32 callsHash = hashCalls(_decoded.calls);
      // The top-level struct for Calls might be something like:
      // Calls(bytes32 callsHash,uint256 space,uint256 nonce,bytes32 walletsHash)
      return keccak256(abi.encode(CALLS_TYPEHASH, callsHash, _decoded.space, _decoded.nonce, walletsHash));
    } else if (_decoded.kind == KIND_MESSAGE) {
      // If you define your top-level as: Message(bytes32 messageHash,bytes32 walletsHash)
      return keccak256(abi.encode(MESSAGE_TYPEHASH, keccak256(_decoded.message), walletsHash));
    } else if (_decoded.kind == KIND_CONFIG_UPDATE) {
      // Top-level: ConfigUpdate(bytes32 imageHash,bytes32 walletsHash)
      return keccak256(abi.encode(CONFIG_UPDATE_TYPEHASH, _decoded.imageHash, walletsHash));
    } else if (_decoded.kind == KIND_DIGEST) {
      // Top-level: Use MESSAGE_TYPEHASH but assume the digest is already the hashed message
      return keccak256(abi.encode(MESSAGE_TYPEHASH, _decoded.digest, walletsHash));
    } else {
      // Unknown kind
      revert InvalidKind(_decoded.kind);
    }
  }

  function hash(
    Decoded memory _decoded
  ) internal view returns (bytes32) {
    bytes32 domain = domainSeparator(_decoded.noChainId, address(this));
    bytes32 structHash = toEIP712(_decoded);
    return keccak256(abi.encodePacked("\x19\x01", domain, structHash));
  }

  function hashFor(Decoded memory _decoded, address _wallet) internal view returns (bytes32) {
    bytes32 domain = domainSeparator(_decoded.noChainId, _wallet);
    bytes32 structHash = toEIP712(_decoded);
    return keccak256(abi.encodePacked("\x19\x01", domain, structHash));
  }

}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

