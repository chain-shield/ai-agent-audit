
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";

struct PriceHistory {
    PriceSnapshot[] snapshots;
}

struct PriceSnapshot {
    uint256 price;
    int256 basisSpread;
    uint256 timestamp;
}

using PriceHistoryLib for PriceHistory global;

library PriceHistoryLib {
    using FixedPointMathLib for uint256;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                SETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice snapshots
    function snapshot(PriceHistory storage history, uint256 price) internal {
        uint256 length = history.snapshots.length;

        if (length > 0 && history.snapshots[length - 1].timestamp == block.timestamp) {
            history.snapshots[length - 1].price = price;
        } else {
            history.snapshots.push(PriceSnapshot(price, 0, block.timestamp));
        }
    }

    function snapshotBasisSpread(PriceHistory storage history, int256 basisSpread) internal {
        history.snapshots.push(PriceSnapshot(0, basisSpread, 0));
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               GETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function latest(PriceHistory storage history) internal view returns (uint256) {
        uint256 length = history.snapshots.length;

        if (length == 0) return 0;

        return history.snapshots[length - 1].price;
    }

    function twap(PriceHistory storage history, uint256 twapInterval) internal view returns (uint256) {
        uint256 idx = history.snapshots.length;

        if (idx == 0) return 0;

        PriceSnapshot memory currentSnapshot = history.snapshots[--idx];

        if (idx == 0) return currentSnapshot.price;

        uint256 targetTime = block.timestamp - twapInterval;
        uint256 timePeriod = block.timestamp - currentSnapshot.timestamp;
        uint256 elapsedTime = timePeriod;
        uint256 weightedPrice = currentSnapshot.price * timePeriod;
        uint256 previousTime = currentSnapshot.timestamp;

        while (currentSnapshot.timestamp > targetTime) {
            // history is too short
            if (idx == 0) break;

            currentSnapshot = history.snapshots[--idx];

            if (currentSnapshot.timestamp < targetTime) {
                // if snapshot is before target time, bound the time period
                elapsedTime += timePeriod = previousTime - targetTime;
            } else {
                elapsedTime += timePeriod = previousTime - currentSnapshot.timestamp;
            }

            weightedPrice += currentSnapshot.price * timePeriod;
            previousTime = currentSnapshot.timestamp;
        }

        return weightedPrice / elapsedTime;
    }

    /// @notice returns ema of basis spread
    function ema(PriceHistory storage history, uint256 period) internal view returns (int256) {
        uint256 n = history.snapshots.length;
        if (n == 0 || period == 0) return 0;

        // only consider up to `period` most recent entries
        uint256 count = period <= n ? period : n;
        uint256 start = n - count;

        int256 k = (2 * 1e18) / (int256(count) + 1);

        // initialize EMA using the first value in the slice (scaled)
        int256 _ema = history.snapshots[start].basisSpread * 1e18;

        // apply EMA formula over the remaining `count - 1` entries
        for (uint256 i = start + 1; i < n; i++) {
            int256 pWad = history.snapshots[i].basisSpread * 1e18;
            _ema = (pWad * k + _ema * (1e18 - k)) / 1e18;
        }

        // Return unscaled EMA value
        return _ema / 1e18;
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES

## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

