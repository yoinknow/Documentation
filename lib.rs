use anchor_lang::prelude::*;
use anchor_lang::system_program::{ transfer, Transfer };
use anchor_spl::token::{ spl_token::instruction::AuthorityType, SetAuthority };

pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

use anchor_spl::{
    associated_token::{ self, AssociatedToken },
    metadata::{ self, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3 },
    token::{ self, Mint, MintTo, Token, TokenAccount },
};

declare_id!("9BSxAV9iRuiT3W7kwhFEkmzfoMo7xZTBdFGRF793JRbC");

pub mod native_mint {
    use anchor_lang::declare_id;

    declare_id!("So11111111111111111111111111111111111111112");
}

pub mod config_feature {
    pub mod withdraw_authority {
        use anchor_lang::declare_id;
        declare_id!("715Zjd5g9kmUMBNBLDQWtbwqCptUrnCaebUfqkEK19rT");
    }

    pub mod platform_authority {
        use anchor_lang::declare_id;
        declare_id!("715Zjd5g9kmUMBNBLDQWtbwqCptUrnCaebUfqkEK19rT");
    }
}

#[program]
pub mod yoink {
    use super::*;

    /// Initialize holder stats account
    pub fn init_holder_stats(ctx: Context<InitHolderStats>) -> Result<()> {
        ctx.accounts.holder_stats.user = ctx.accounts.user.key();
        ctx.accounts.holder_stats.mint = ctx.accounts.mint.key();
        ctx.accounts.holder_stats.current_balance = ctx.accounts.associated_user.amount;
        ctx.accounts.holder_stats.fees_claimed = 0;
        ctx.accounts.holder_stats.entry_position = 0; // Will be set on first buy
        ctx.accounts.holder_stats.total_volume = 0;

        Ok(())
    }

    /// Update holder stats and top holders list
    pub fn update_holder_stats(ctx: Context<UpdateHolderStats>) -> Result<()> {
        // Update holder stats
        ctx.accounts.holder_stats.current_balance = ctx.accounts.associated_user.amount;

        Ok(())
    }

    /// Creates the global state.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        require!(!ctx.accounts.global.initialized, HorseFunError::AlreadyInitialized);

        ctx.accounts.global.authority = *ctx.accounts.user.key;
        ctx.accounts.global.initialized = true;
        ctx.accounts.global.buybacks_enabled = true;
        Ok(())
    }

    /// Sets the global state parameters.
    pub fn set_params(
        ctx: Context<SetParams>,
        fee_recipient: Pubkey,
        initial_virtual_token_reserves: u64,
        initial_virtual_sol_reserves: u64,
        initial_real_token_reserves: u64,
        token_total_supply: u64,
        fee_basis_points: u64,
        creator_fee_share: u64,
        platform_fee_share: u64,
        treasury_fee_share: u64,
        early_bird_fee_share: u64,
        buybacks_enabled: bool,
        buyback_params: BuybackParams,
        early_bird_enabled: bool,
        early_bird_cutoff: u64,
        early_bird_min_buy_sol: u64
    ) -> Result<()> {
        require!(ctx.accounts.global.initialized, HorseFunError::NotInitialized);
        require_keys_eq!(
            ctx.accounts.user.key(),
            ctx.accounts.global.authority,
            HorseFunError::NotAuthorized
        );

        // Validate fee shares add up to 100%
        require!(
            creator_fee_share + platform_fee_share + treasury_fee_share + early_bird_fee_share ==
                10000,
            HorseFunError::InvalidFeeShares
        );

        ctx.accounts.global.fee_recipient = fee_recipient;
        ctx.accounts.global.initial_virtual_token_reserves = initial_virtual_token_reserves;
        ctx.accounts.global.initial_virtual_sol_reserves = initial_virtual_sol_reserves;
        ctx.accounts.global.initial_real_token_reserves = initial_real_token_reserves;
        ctx.accounts.global.token_total_supply = token_total_supply;
        ctx.accounts.global.fee_basis_points = fee_basis_points;
        ctx.accounts.global.creator_fee_share = creator_fee_share;
        ctx.accounts.global.platform_fee_share = platform_fee_share;
        ctx.accounts.global.treasury_fee_share = treasury_fee_share;
        ctx.accounts.global.early_bird_fee_share = early_bird_fee_share;
        ctx.accounts.global.buybacks_enabled = buybacks_enabled;
        ctx.accounts.global.buyback_params = buyback_params;
        ctx.accounts.global.early_bird_enabled = early_bird_enabled;
        ctx.accounts.global.early_bird_cutoff = early_bird_cutoff;
        ctx.accounts.global.early_bird_min_buy_sol = early_bird_min_buy_sol;

        emit_cpi!(SetParamsEvent {
            fee_recipient,
            initial_virtual_token_reserves,
            initial_virtual_sol_reserves,
            initial_real_token_reserves,
            token_total_supply,
            fee_basis_points,
            creator_fee_share,
            platform_fee_share,
            treasury_fee_share,
            buybacks_enabled,
        });

        emit!(SetParamsEvent {
            fee_recipient,
            initial_virtual_token_reserves,
            initial_virtual_sol_reserves,
            initial_real_token_reserves,
            token_total_supply,
            fee_basis_points,
            creator_fee_share,
            platform_fee_share,
            treasury_fee_share,
            buybacks_enabled,
        });

        Ok(())
    }

    /// Creates a new coin and bonding curve.
    pub fn create(
        ctx: Context<Create>,
        name: String,
        symbol: String,
        uri: String,
        streamer_id: Option<String>
    ) -> Result<()> {
        msg!("Creating new token:");
        msg!(" - Creator Wallet: {}", ctx.accounts.user.key());
        msg!(" - Mint Address: {}", ctx.accounts.mint.key());

        // Log streamer ID details
        match &streamer_id {
            Some(id) => {
                msg!(" - Streamer ID provided: {}", id);
                require!(!id.is_empty() && id.len() <= 50, HorseFunError::InvalidStreamerId);
                msg!(" - Streamer ID validation passed");
            }
            None => msg!(" - No Streamer ID provided, using wallet-only verification"),
        }

        // initialize the bonding curve parameters
        ctx.accounts.bonding_curve.virtual_token_reserves =
            ctx.accounts.global.initial_virtual_token_reserves;
        ctx.accounts.bonding_curve.virtual_sol_reserves =
            ctx.accounts.global.initial_virtual_sol_reserves;
        ctx.accounts.bonding_curve.real_token_reserves =
            ctx.accounts.global.initial_real_token_reserves;
        ctx.accounts.bonding_curve.real_sol_reserves = 0;
        ctx.accounts.bonding_curve.token_total_supply = ctx.accounts.global.token_total_supply;
        ctx.accounts.bonding_curve.circulating_supply = ctx.accounts.global.token_total_supply;
        ctx.accounts.bonding_curve.complete = false;
        ctx.accounts.bonding_curve.total_burned_supply = 0;
        ctx.accounts.bonding_curve.total_treasury_spent = 0;

        // Set creator info
        ctx.accounts.bonding_curve.creator_wallet = ctx.accounts.user.key();
        ctx.accounts.bonding_curve.creator_streamer_id = streamer_id;
        ctx.accounts.bonding_curve.creator_fee_pool = 0;
        ctx.accounts.bonding_curve.treasury_fee_pool = 0;
        ctx.accounts.bonding_curve.total_fees_accrued = 0;
        ctx.accounts.bonding_curve.total_treasury_fees_accrued = 0;
        ctx.accounts.bonding_curve.ema_lot_price = 0;

        // Initialize early bird fields
        ctx.accounts.bonding_curve.early_bird_pool = 0;
        ctx.accounts.bonding_curve.total_buyers = 0;
        ctx.accounts.bonding_curve.total_early_bird_fees_accrued = 0;
        // Initialize with cutoff value - this will be decremented when early birds sell
        ctx.accounts.bonding_curve.early_bird_valid_count = 0;
        ctx.accounts.bonding_curve.early_bird_share_per_seat = 0; // Will be set when curve completes

        // set the metadata for the token
        helpers::set_metadata(&ctx, name.clone(), symbol.clone(), uri.clone())?;

        // mint tokens to the bonding curve
        helpers::mint_to_bonding_curve(&ctx)?;

        // revoke the mint authority
        helpers::revoke_mint_authority(&ctx)?;

        emit_cpi!(CreateEvent {
            name: name.clone(),
            symbol: symbol.clone(),
            uri: uri.clone(),
            mint: ctx.accounts.mint.key(),
            bonding_curve: ctx.accounts.bonding_curve.key(),
            user: ctx.accounts.user.key(),
        });

        emit!(CreateEvent {
            name,
            symbol,
            uri,
            mint: ctx.accounts.mint.key(),
            bonding_curve: ctx.accounts.bonding_curve.key(),
            user: ctx.accounts.user.key(),
        });

        Ok(())
    }

    /// Buys tokens from a bonding curve.
    pub fn buy(mut ctx: Context<Buy>, amount: u64, max_sol_cost: u64) -> Result<()> {
        // Log initial fee state
        msg!("Buy: Fee state before trade:");
        msg!(" - Creator Fee Pool: {}", ctx.accounts.bonding_curve.creator_fee_pool);
        msg!(" - Treasury Fee Pool: {}", ctx.accounts.bonding_curve.treasury_fee_pool);
        msg!(" - Total Creator Fees Accrued: {}", ctx.accounts.bonding_curve.total_fees_accrued);
        msg!(
            " - Total Treasury Fees Accrued: {}",
            ctx.accounts.bonding_curve.total_treasury_fees_accrued
        );

        // Cap amount to available reserves
        let available_amount = std::cmp::min(
            amount,
            ctx.accounts.bonding_curve.real_token_reserves
        );

        // Calculate price for the capped amount
        let sol_cost = ctx.accounts.bonding_curve.buy_quote(available_amount as u128);
        let fee = ctx.accounts.global.get_fee(sol_cost);

        // Check slippage
        require!(sol_cost + fee <= max_sol_cost, HorseFunError::TooMuchSolRequired);
        require_keys_eq!(
            ctx.accounts.associated_bonding_curve.mint,
            ctx.accounts.mint.key(),
            HorseFunError::MintDoesNotMatchBondingCurve
        );
        require!(!ctx.accounts.bonding_curve.complete, HorseFunError::BondingCurveComplete);

        msg!(" - Amount: {}", amount);
        msg!(" - virtual token reserves: {}", ctx.accounts.bonding_curve.virtual_token_reserves);
        msg!(" - Real Token Reserves: {}", ctx.accounts.bonding_curve.real_token_reserves);

        // Update virtual reserves (these track the theoretical price curve, including fees)
        ctx.accounts.bonding_curve.virtual_token_reserves -= available_amount;
        ctx.accounts.bonding_curve.virtual_sol_reserves += sol_cost;

        // Update real reserves (actual tokens and SOL in the curve)
        ctx.accounts.bonding_curve.real_token_reserves -= available_amount;
        // Only add the actual SOL cost to reserves, fees are tracked separately in fee pools
        ctx.accounts.bonding_curve.real_sol_reserves += sol_cost;

        if ctx.accounts.bonding_curve.real_token_reserves == 0 {
            ctx.accounts.bonding_curve.complete = true;

            // ⭐ Calculate and cache equal share for early bird rewards
            // This ensures all early birds get EXACTLY the same amount
            let valid_count = ctx.accounts.bonding_curve.early_bird_valid_count;
            if valid_count > 0 && ctx.accounts.bonding_curve.early_bird_pool > 0 {
                ctx.accounts.bonding_curve.early_bird_share_per_seat =
                    ctx.accounts.bonding_curve.early_bird_pool / valid_count;
                msg!(
                    "🐦 Early Bird rewards locked: {} valid seats, {} lamports per seat",
                    valid_count,
                    ctx.accounts.bonding_curve.early_bird_share_per_seat
                );
            }

            emit_cpi!(CompleteEvent {
                mint: ctx.accounts.mint.key(),
                user: ctx.accounts.user.key(),
                bonding_curve: ctx.accounts.bonding_curve.key(),
                timestamp: Clock::get()?.unix_timestamp,
                early_bird_pool: ctx.accounts.bonding_curve.early_bird_pool,
            });

            emit!(CompleteEvent {
                mint: ctx.accounts.mint.key(),
                user: ctx.accounts.user.key(),
                bonding_curve: ctx.accounts.bonding_curve.key(),
                timestamp: Clock::get()?.unix_timestamp,
                early_bird_pool: ctx.accounts.bonding_curve.early_bird_pool,
            });
        }

        // transfer the capped amount using the helper
        helpers::transfer_tokens_from_bonding_curve_to_user(&ctx, available_amount)?;

        // transfer the sol from the user to the bonding curve (only the actual cost, not including fee)
        helpers::transfer_sol_from_user_to_bonding_curve(&ctx, sol_cost)?;

        // transfer the fee separately from user to fee recipient and update fee pools
        helpers::transfer_sol_from_user_to_fee_recipient(&mut ctx, fee)?;

        // Log final fee state
        msg!("Buy: Fee state after trade:");
        msg!(" - Creator Fee Pool: {}", ctx.accounts.bonding_curve.creator_fee_pool);
        msg!(" - Treasury Fee Pool: {}", ctx.accounts.bonding_curve.treasury_fee_pool);
        msg!(" - Early Bird Pool: {}", ctx.accounts.bonding_curve.early_bird_pool);
        msg!(" - Total Creator Fees Accrued: {}", ctx.accounts.bonding_curve.total_fees_accrued);
        msg!(
            " - Total Treasury Fees Accrued: {}",
            ctx.accounts.bonding_curve.total_treasury_fees_accrued
        );

        // Update holder stats and track entry position for early bird rewards
        ctx.accounts.holder_stats.current_balance = ctx.accounts.holder_stats.current_balance
            .checked_add(available_amount)
            .ok_or(HorseFunError::ArithmeticOverflow)?;
        ctx.accounts.holder_stats.total_volume += sol_cost;

        // Track entry position if this is the first buy (for early bird rewards)
        // Note: entry_position == 0 means first-time buyer
        //       entry_position == u64::MAX means revoked/disqualified (sold before)
        // Anti-Sybil: Requires minimum SOL buy amount to prevent 50-wallet gaming
        if ctx.accounts.holder_stats.entry_position == 0 {
            // Check if buy amount meets minimum threshold for Early Bird eligibility
            if sol_cost >= ctx.accounts.global.early_bird_min_buy_sol {
                ctx.accounts.bonding_curve.total_buyers += 1;
                ctx.accounts.holder_stats.entry_position = ctx.accounts.bonding_curve.total_buyers;

                // Log early bird status
                if
                    ctx.accounts.global.early_bird_enabled &&
                    ctx.accounts.holder_stats.entry_position <=
                        ctx.accounts.global.early_bird_cutoff
                {
                    ctx.accounts.bonding_curve.early_bird_valid_count += 1;
                    msg!(
                        "🐦 Early Bird #{}/{}! User will earn rewards from future trades!",
                        ctx.accounts.holder_stats.entry_position,
                        ctx.accounts.global.early_bird_cutoff
                    );
                }
            } else {
                msg!(
                    "⚠️ Buy amount {} lamports is below minimum {} lamports for Early Bird eligibility",
                    sol_cost,
                    ctx.accounts.global.early_bird_min_buy_sol
                );
            }
        } else if ctx.accounts.holder_stats.entry_position == u64::MAX {
            msg!("🚫 User is permanently disqualified from Early Bird rewards (sold previously)");
        }

        // Initialize buyback-related values with defaults (no buyback occurred yet)
        let mut is_buyback = false;
        let mut burn_amount = 0;
        let mut price_lamports_per_token = 0;

        if ctx.accounts.global.buybacks_enabled {
            // ---- Dynamic (EMA + Backing) Buyback: lot-based, with verbose logs & safer caps ----
            const DECIMALS: u32 = 6; // your mint decimals
            const LOT_SIZE: u64 = (10u64).pow(DECIMALS); // 1 whole token

            let gp = ctx.accounts.global.buyback_params;
            let backing_mult_bps = gp.backing_mult_bps as u64;
            let ema_drop_bps = gp.ema_drop_bps as u64;
            let ema_alpha_bps = gp.ema_alpha_bps as u64;
            let spend_bps = gp.spend_bps as u64;
            let max_supply_bps = gp.max_supply_bps as u64;
            let min_backing_lamports = gp.min_backing_lamports; // ⭐ DON'T override this later!
            let max_burn_pct = gp.max_burn_percentage_bps as u64;

            // Check if we've already hit the maximum burn limit (TOTAL cumulative burns, not per-tx)
            let total_supply = ctx.accounts.bonding_curve.token_total_supply;
            let already_burned = ctx.accounts.bonding_curve.total_burned_supply;
            let current_burn_pct = if total_supply > 0 {
                // Use u128 to prevent overflow on large token supplies
                (((already_burned as u128) * 10_000) / (total_supply as u128)) as u64
            } else {
                0
            };

            if current_burn_pct >= max_burn_pct {
                msg!(
                    "BB[LIMIT]: Max burn percentage reached: {}bps / {}bps ({}% / {}%)",
                    current_burn_pct,
                    max_burn_pct,
                    current_burn_pct / 100,
                    max_burn_pct / 100
                );
                msg!("BB[LIMIT]: Total burned: {} / {}", already_burned, total_supply);
            } else {
                msg!(
                    "BB[burn-status]: {}/{}bps used ({}% / {}%)",
                    current_burn_pct,
                    max_burn_pct,
                    current_burn_pct / 100,
                    max_burn_pct / 100
                );

                let vt = ctx.accounts.bonding_curve.virtual_token_reserves;
                let vsol = ctx.accounts.bonding_curve.virtual_sol_reserves;
                let rtok = ctx.accounts.bonding_curve.real_token_reserves;
                let tpool = ctx.accounts.bonding_curve.treasury_fee_pool;

                msg!("BB[state]: vt={} vsol={} rtok={} tpool={}", vt, vsol, rtok, tpool);

                if vt > 1 && tpool > 0 {
                    // 1) Quote a *lot*, not 1 atomic unit
                    let lot = LOT_SIZE.min(vt.saturating_sub(1));
                    match ctx.accounts.bonding_curve.buy_quote_checked(lot) {
                        None => {
                            msg!("BB[skip]: unsafe lot quote (lot={} >= vT={})", lot, vt);
                        }
                        Some(market_lot) => {
                            // 2) Compute backing per lot with treasury
                            let backing_lot =
                                ctx.accounts.bonding_curve.backing_per_lot_with_treasury(
                                    lot,
                                    tpool
                                );

                            // 3) Update EMA, then read it
                            ctx.accounts.bonding_curve.update_ema_lot_price(
                                market_lot,
                                ema_alpha_bps
                            );
                            let ema_lot = ctx.accounts.bonding_curve.ema_lot_price;

                            // 4) Build trigger thresholds
                            let backing_thr = (((backing_lot as u128) *
                                (backing_mult_bps as u128)) /
                                10_000u128) as u64;
                            let ema_thr = (((ema_lot as u128) * (ema_drop_bps as u128)) /
                                10_000u128) as u64;
                            let trigger_thr = backing_thr.max(ema_thr);

                            msg!(
                                "BB[chk]: lot={} mkt={} bkt={} ema={} thr={} tpool={}",
                                lot,
                                market_lot,
                                backing_lot,
                                ema_lot,
                                trigger_thr,
                                tpool
                            );

                            // 5) Trigger condition - balanced approach
                            // Allow buybacks when price is below threshold AND
                            // either backing is above a minimum OR the price drop is significant
                            // NOTE: min_backing_lamports loaded from params above, dont override it here!
                            let significant_drop = market_lot <= ema_lot / 2; // Price dropped by 50% or more from EMA

                            if
                                market_lot <= trigger_thr &&
                                (backing_lot >= min_backing_lamports.into() || significant_drop)
                            {
                                // Budget from treasury (lamports)
                                let budget = (((tpool as u128) * (spend_bps as u128)) /
                                    10_000u128) as u64;
                                msg!("BB[budget]: spend_bps={} budget={}", spend_bps, budget);

                                // Convert budget → tokens via CPMM inversion
                                let mut amount =
                                    ctx.accounts.bonding_curve.tokens_for_budget(budget);
                                msg!("BB[size0]: tokens_for_budget -> {}", amount);

                                // Cap to ≤10% of on-curve tokens; ensure it never floors to 0 when rtok>0
                                let mut max_supply_buy = if rtok == 0 {
                                    0
                                } else {
                                    (rtok.saturating_mul(max_supply_bps) / 10_000).max(1)
                                };
                                if max_supply_buy > rtok {
                                    max_supply_buy = rtok;
                                }

                                msg!(
                                    "BB[caps]: rtok={} cap_bps={} cap_tokens={}",
                                    rtok,
                                    max_supply_bps,
                                    max_supply_buy
                                );

                                if max_supply_buy > 0 {
                                    amount = amount.min(max_supply_buy);
                                    msg!("BB[size1]: after cap -> {}", amount);
                                } else {
                                    msg!("BB[warn]: cap_tokens=0 (rtok=0), cannot buy");
                                }

                                // Guard rounding to zero → try minimum 1 atomic unit if affordable
                                if amount == 0 {
                                    match ctx.accounts.bonding_curve.buy_quote_checked(1) {
                                        Some(min_cost) => {
                                            msg!(
                                                "BB[fallback-atomic]: min_cost={} tpool={}",
                                                min_cost,
                                                tpool
                                            );
                                            if tpool >= min_cost && max_supply_buy >= 1 {
                                                amount = 1;
                                                msg!("BB[fallback-atomic]: amount set to 1");
                                            } else {
                                                msg!(
                                                    "BB[fallback-atomic]: insufficient tpool or cap<1"
                                                );
                                            }
                                        }
                                        None => msg!("BB[fallback-atomic]: unsafe quote(1)"),
                                    }
                                }

                                // Optional second fallback: try one *lot* if affordable and within caps
                                if amount == 0 && lot > 0 && max_supply_buy >= lot {
                                    match ctx.accounts.bonding_curve.buy_quote_checked(lot) {
                                        Some(lot_cost) => {
                                            msg!(
                                                "BB[fallback-lot]: lot={} cost={} tpool={}",
                                                lot,
                                                lot_cost,
                                                tpool
                                            );
                                            if tpool >= lot_cost {
                                                amount = lot.min(max_supply_buy);
                                                msg!("BB[fallback-lot]: amount set to {}", amount);
                                            } else {
                                                msg!(
                                                    "BB[fallback-lot]: insufficient tpool for lot"
                                                );
                                            }
                                        }
                                        None => msg!("BB[fallback-lot]: unsafe lot quote"),
                                    }
                                }

                                if amount > 0 {
                                    // Final quote & funds check
                                    match ctx.accounts.bonding_curve.buy_quote_checked(amount) {
                                        None => {
                                            msg!(
                                                "BB[skip]: unsafe quote for amount={} (vT={})",
                                                amount,
                                                ctx.accounts.bonding_curve.virtual_token_reserves
                                            );
                                        }
                                        Some(bb_cost) => {
                                            msg!(
                                                "BB[quote]: amount={} bb_cost={} tpool={}",
                                                amount,
                                                bb_cost,
                                                tpool
                                            );
                                            require!(
                                                tpool >= bb_cost,
                                                HorseFunError::InsufficientTreasuryFunds
                                            );

                                            // Snapshot pre-state for delta logs
                                            let vtr0 =
                                                ctx.accounts.bonding_curve.virtual_token_reserves;
                                            let rtr0 =
                                                ctx.accounts.bonding_curve.real_token_reserves;
                                            let vsr0 =
                                                ctx.accounts.bonding_curve.virtual_sol_reserves;
                                            let rsr0 = ctx.accounts.bonding_curve.real_sol_reserves;
                                            let tpool0 =
                                                ctx.accounts.bonding_curve.treasury_fee_pool;

                                            // Spend from treasury pool (book-keeping)
                                            ctx.accounts.bonding_curve.treasury_fee_pool =
                                                ctx.accounts.bonding_curve.treasury_fee_pool.saturating_sub(
                                                    bb_cost
                                                );

                                            // Apply the same state transition as a buy:
                                            ctx.accounts.bonding_curve.virtual_token_reserves =
                                                ctx.accounts.bonding_curve.virtual_token_reserves.saturating_sub(
                                                    amount
                                                );
                                            ctx.accounts.bonding_curve.virtual_sol_reserves =
                                                ctx.accounts.bonding_curve.virtual_sol_reserves.saturating_add(
                                                    bb_cost
                                                );

                                            ctx.accounts.bonding_curve.real_token_reserves =
                                                ctx.accounts.bonding_curve.real_token_reserves.saturating_sub(
                                                    amount
                                                );
                                            ctx.accounts.bonding_curve.real_sol_reserves =
                                                ctx.accounts.bonding_curve.real_sol_reserves.saturating_add(
                                                    bb_cost
                                                );

                                            // Intended burn sizing = full buyback size
                                            burn_amount = amount;
                                            let curve_ata_bal =
                                                ctx.accounts.associated_bonding_curve.amount;
                                            let mint_supply_before = ctx.accounts.mint.supply;

                                            msg!(
                                                "BB[burn-plan]: burn_amount={} curve_ata_bal={} mint_supply_before={}",
                                                burn_amount,
                                                curve_ata_bal,
                                                mint_supply_before
                                            );

                                            // Safety guard
                                            let burn_ok =
                                                burn_amount > 0 && curve_ata_bal >= burn_amount;
                                            msg!(
                                                "BB[burn-guard]: curve_ata>=intended?={} ({}>=?)",
                                                burn_ok,
                                                curve_ata_bal
                                            );

                                            // ---- BURN ENABLED ----
                                            if burn_ok {
                                                {
                                                    // Isolated scope to avoid borrow conflicts during CPI
                                                    helpers::burn_from_curve_ata_on_buy(
                                                        &ctx,
                                                        burn_amount
                                                    )?;
                                                }

                                                // IMPORTANT: do NOT subtract real_token_reserves again here.
                                                // We already reduced it by `amount` when applying the buyback math above.
                                                // Update circulating supply when tokens are burned
                                                ctx.accounts.bonding_curve.circulating_supply =
                                                    ctx.accounts.bonding_curve.circulating_supply.saturating_sub(
                                                        burn_amount
                                                    );
                                            }

                                            // track totals
                                            ctx.accounts.bonding_curve.total_treasury_spent =
                                                ctx.accounts.bonding_curve.total_treasury_spent.saturating_add(
                                                    bb_cost
                                                );

                                            let mut burned: u64 = 0;
                                            if burn_ok {
                                                // burn already executed above
                                                burned = burn_amount;
                                                ctx.accounts.bonding_curve.total_burned_supply =
                                                    ctx.accounts.bonding_curve.total_burned_supply.saturating_add(
                                                        burn_amount
                                                    );
                                            }

                                            let price_lpt = if amount > 0 {
                                                bb_cost.saturating_div(amount)
                                            } else {
                                                0
                                            };

                                            // Update buyback variables for TradeEvent at end of function
                                            is_buyback = true;
                                            burn_amount = burned;
                                            price_lamports_per_token = price_lpt;

                                            // Note: TradeEvent will be emitted at end of function

                                            // Delta log
                                            msg!(
                                                "BB[go]: amount={} cost={} new_tpool={}",
                                                amount,
                                                bb_cost,
                                                ctx.accounts.bonding_curve.treasury_fee_pool
                                            );
                                            msg!(
                                                "BB[delta]: Δvtok=-{} Δrtok=-{} Δvsol=+{} Δrsol=+{} Δtpool=-{}",
                                                vtr0.saturating_sub(
                                                    ctx.accounts.bonding_curve.virtual_token_reserves
                                                ),
                                                rtr0.saturating_sub(
                                                    ctx.accounts.bonding_curve.real_token_reserves
                                                ),
                                                ctx.accounts.bonding_curve.virtual_sol_reserves.saturating_sub(
                                                    vsr0
                                                ),
                                                ctx.accounts.bonding_curve.real_sol_reserves.saturating_sub(
                                                    rsr0
                                                ),
                                                tpool0.saturating_sub(
                                                    ctx.accounts.bonding_curve.treasury_fee_pool
                                                )
                                            );
                                        }
                                    }
                                } else {
                                    msg!("BB[skip]: amount=0 after sizing and fallbacks");
                                }
                            } else {
                                msg!(
                                    "BB[skip]: no trigger mkt={} thr={} bkt={}",
                                    market_lot,
                                    trigger_thr,
                                    backing_lot
                                );
                            }
                        }
                    }
                }
            } // Close the else block for burn limit check
        } else {
            msg!("Buybacks are disabled globally");
        }

        // Calculate fee splits for this trade to emit in event
        let (_, creator_fee_for_trade, _, _) = ctx.accounts.global.get_fee_splits(fee);

        // 🔧 Calculate if this user is a valid early bird
        let user_pos = ctx.accounts.holder_stats.entry_position;
        let is_early_bird =
            user_pos > 0 &&
            user_pos != u64::MAX &&
            user_pos <= ctx.accounts.global.early_bird_cutoff;

        // Emit final TradeEvent with all information (including possible buyback)
        let trade_event = TradeEvent {
            user: ctx.accounts.user.key(),
            sol_amount: sol_cost,
            token_amount: available_amount,
            is_buy: true,
            mint: ctx.accounts.mint.key(),
            timestamp: Clock::get()?.unix_timestamp,
            virtual_sol_reserves: ctx.accounts.bonding_curve.virtual_sol_reserves,
            virtual_token_reserves: ctx.accounts.bonding_curve.virtual_token_reserves,
            circulating_supply: ctx.accounts.bonding_curve.circulating_supply,
            real_token_reserves: ctx.accounts.bonding_curve.real_token_reserves,
            real_sol_reserves: ctx.accounts.bonding_curve.real_sol_reserves,
            creator_fee_pool: ctx.accounts.bonding_curve.creator_fee_pool,
            treasury_fee_pool: ctx.accounts.bonding_curve.treasury_fee_pool,
            total_fees_accrued: ctx.accounts.bonding_curve.total_fees_accrued,
            total_treasury_fees_accrued: ctx.accounts.bonding_curve.total_treasury_fees_accrued,
            creator_fee_amount: creator_fee_for_trade, // Fee earned by creator from THIS trade
            fee_recipient: ctx.accounts.bonding_curve.creator_wallet, // Current creator (CTO-aware)
            is_buyback,
            burn_amount,
            price_lamports_per_token,
            total_burned_supply: ctx.accounts.bonding_curve.total_burned_supply,
            total_treasury_spent: ctx.accounts.bonding_curve.total_treasury_spent,
            early_bird_pool: ctx.accounts.bonding_curve.early_bird_pool,
            total_early_bird_fees_accrued: ctx.accounts.bonding_curve.total_early_bird_fees_accrued,
            user_position: ctx.accounts.holder_stats.entry_position,
            user_balance: ctx.accounts.holder_stats.current_balance,
            early_bird_cutoff: ctx.accounts.global.early_bird_cutoff,
            total_buyers: ctx.accounts.bonding_curve.total_buyers,
            early_bird_valid_count: ctx.accounts.bonding_curve.early_bird_valid_count,
            is_early_bird,
        };

        emit_cpi!(trade_event);
        emit!(trade_event);

        Ok(())
    }

    /// Sells tokens into a bonding curve.
    pub fn sell(mut ctx: Context<Sell>, amount: u64, min_sol_output: u64) -> Result<()> {
        // Log initial fee state
        msg!("Sell: Fee state before trade:");
        msg!(" - Creator Fee Pool: {}", ctx.accounts.bonding_curve.creator_fee_pool);
        msg!(" - Treasury Fee Pool: {}", ctx.accounts.bonding_curve.treasury_fee_pool);
        msg!(" - Total Creator Fees Accrued: {}", ctx.accounts.bonding_curve.total_fees_accrued);
        msg!(
            " - Total Treasury Fees Accrued: {}",
            ctx.accounts.bonding_curve.total_treasury_fees_accrued
        );

        // Initialize buyback-related values with defaults (no buyback occurred yet)
        let mut is_buyback = false;
        let mut burn_amount = 0;
        let mut price_lamports_per_token = 0;

        let sol_output = ctx.accounts.bonding_curve.sell_quote(amount as u128);
        let fee = ctx.accounts.global.get_fee(sol_output);

        // check that the sol cost is within the slippage tolerance
        require!(
            sol_output.saturating_sub(fee) >= min_sol_output,
            HorseFunError::TooLittleSolReceived
        );
        require_keys_eq!(
            ctx.accounts.associated_bonding_curve.mint,
            ctx.accounts.mint.key(),
            HorseFunError::MintDoesNotMatchBondingCurve
        );
        require!(!ctx.accounts.bonding_curve.complete, HorseFunError::BondingCurveComplete);

        // update the bonding curve parameters (excluding fee)
        ctx.accounts.bonding_curve.virtual_token_reserves =
            ctx.accounts.bonding_curve.virtual_token_reserves.saturating_add(amount);
        ctx.accounts.bonding_curve.real_token_reserves =
            ctx.accounts.bonding_curve.real_token_reserves.saturating_add(amount);
        ctx.accounts.bonding_curve.virtual_sol_reserves =
            ctx.accounts.bonding_curve.virtual_sol_reserves.saturating_sub(sol_output);
        // Only reduce by actual transfer amount (user receives sol_output - fee)
        ctx.accounts.bonding_curve.real_sol_reserves =
            ctx.accounts.bonding_curve.real_sol_reserves.saturating_sub(
                sol_output.saturating_sub(fee)
            );

        // transfer the tokens from the user to the bonding curve
        helpers::transfer_tokens_from_user_to_bonding_curve(&ctx, amount)?;

        // Log final fee state
        msg!("Sell: Fee state after trade:");
        msg!(" - Creator Fee Pool: {}", ctx.accounts.bonding_curve.creator_fee_pool);
        msg!(" - Treasury Fee Pool: {}", ctx.accounts.bonding_curve.treasury_fee_pool);
        msg!(" - Early Bird Pool: {}", ctx.accounts.bonding_curve.early_bird_pool);
        msg!(" - Total Creator Fees Accrued: {}", ctx.accounts.bonding_curve.total_fees_accrued);
        msg!(
            " - Total Treasury Fees Accrued: {}",
            ctx.accounts.bonding_curve.total_treasury_fees_accrued
        );

        // Calculate new balance by subtracting the tokens just sold from the current balance
        // (Anchor's cached account might not reflect the latest transfer yet)
        ctx.accounts.holder_stats.current_balance = ctx.accounts.holder_stats.current_balance
            .checked_sub(amount)
            .ok_or(HorseFunError::ArithmeticOverflow)?;
        helpers::revoke_early_bird_status(
            &mut ctx.accounts.holder_stats,
            &mut ctx.accounts.bonding_curve,
            &ctx.accounts.global
        );

        // Process buyback if enabled
        if ctx.accounts.global.buybacks_enabled {
            // ---- Dynamic (EMA + Backing) Buyback: lot-based, with verbose logs & safer caps ----
            const DECIMALS: u32 = 6; // your mint decimals
            const LOT_SIZE: u64 = (10u64).pow(DECIMALS); // 1 whole token

            let gp = ctx.accounts.global.buyback_params;
            let backing_mult_bps = gp.backing_mult_bps as u64;
            let ema_drop_bps = gp.ema_drop_bps as u64;
            let ema_alpha_bps = gp.ema_alpha_bps as u64;
            let spend_bps = gp.spend_bps as u64;
            let max_supply_bps = gp.max_supply_bps as u64;
            let min_backing_lamports = gp.min_backing_lamports; // ⭐ DONT override this later!
            let max_burn_pct = gp.max_burn_percentage_bps as u64;

            // Check if we've already hit the maximum burn limit (TOTAL cumulative burns, not per-tx)
            let total_supply = ctx.accounts.bonding_curve.token_total_supply;
            let already_burned = ctx.accounts.bonding_curve.total_burned_supply;
            let current_burn_pct = if total_supply > 0 {
                // Use u128 to prevent overflow on large token supplies
                (((already_burned as u128) * 10_000) / (total_supply as u128)) as u64
            } else {
                0
            };

            if current_burn_pct >= max_burn_pct {
                msg!(
                    "SBB[LIMIT]: Max burn percentage reached: {}bps / {}bps ({}% / {}%)",
                    current_burn_pct,
                    max_burn_pct,
                    current_burn_pct / 100,
                    max_burn_pct / 100
                );
                msg!("SBB[LIMIT]: Total burned: {} / {}", already_burned, total_supply);
            } else {
                msg!(
                    "SBB[burn-status]: {}/{}bps used ({}% / {}%)",
                    current_burn_pct,
                    max_burn_pct,
                    current_burn_pct / 100,
                    max_burn_pct / 100
                );

                let vt = ctx.accounts.bonding_curve.virtual_token_reserves;
                let vsol = ctx.accounts.bonding_curve.virtual_sol_reserves;
                let rtok = ctx.accounts.bonding_curve.real_token_reserves;
                let tpool = ctx.accounts.bonding_curve.treasury_fee_pool;

                msg!("BB[state]: vt={} vsol={} rtok={} tpool={}", vt, vsol, rtok, tpool);

                if vt > 1 && tpool > 0 {
                    // 1) Quote a *lot*, not 1 atomic unit
                    let lot = LOT_SIZE.min(vt.saturating_sub(1));
                    match ctx.accounts.bonding_curve.buy_quote_checked(lot) {
                        None => {
                            msg!("BB[skip]: unsafe lot quote (lot={} >= vT={})", lot, vt);
                        }
                        Some(market_lot) => {
                            // 2) Compute backing per lot with treasury
                            let backing_lot =
                                ctx.accounts.bonding_curve.backing_per_lot_with_treasury(
                                    lot,
                                    tpool
                                );

                            // 3) Update EMA, then read it
                            ctx.accounts.bonding_curve.update_ema_lot_price(
                                market_lot,
                                ema_alpha_bps
                            );
                            let ema_lot = ctx.accounts.bonding_curve.ema_lot_price;

                            // 4) Build trigger thresholds
                            let backing_thr = (((backing_lot as u128) *
                                (backing_mult_bps as u128)) /
                                10_000u128) as u64;
                            let ema_thr = (((ema_lot as u128) * (ema_drop_bps as u128)) /
                                10_000u128) as u64;
                            let trigger_thr = backing_thr.max(ema_thr);

                            msg!(
                                "BB[chk]: lot={} mkt={} bkt={} ema={} thr={} tpool={}",
                                lot,
                                market_lot,
                                backing_lot,
                                ema_lot,
                                trigger_thr,
                                tpool
                            );

                            // 5) Trigger condition - balanced approach
                            // Allow buybacks when price is below threshold AND
                            // either backing is above a minimum OR the price drop is significant
                            // NOTE: min_backing_lamports loaded from params above, dont override it here!
                            let significant_drop = market_lot <= ema_lot / 2; // Price dropped by 50% or more from EMA

                            if
                                market_lot <= trigger_thr &&
                                (backing_lot >= min_backing_lamports.into() || significant_drop)
                            {
                                // Budget from treasury (lamports)
                                let budget = (((tpool as u128) * (spend_bps as u128)) /
                                    10_000u128) as u64;
                                msg!("BB[budget]: spend_bps={} budget={}", spend_bps, budget);

                                // Convert budget → tokens via CPMM inversion
                                let mut amount =
                                    ctx.accounts.bonding_curve.tokens_for_budget(budget);
                                msg!("BB[size0]: tokens_for_budget -> {}", amount);

                                // Cap to ≤10% of on-curve tokens; ensure it never floors to 0 when rtok>0
                                let mut max_supply_buy = if rtok == 0 {
                                    0
                                } else {
                                    (rtok.saturating_mul(max_supply_bps) / 10_000).max(1)
                                };
                                // Never exceed available curve inventory
                                if max_supply_buy > rtok {
                                    max_supply_buy = rtok;
                                }

                                msg!(
                                    "BB[caps]: rtok={} cap_bps={} cap_tokens={}",
                                    rtok,
                                    max_supply_bps,
                                    max_supply_buy
                                );

                                if max_supply_buy > 0 {
                                    amount = amount.min(max_supply_buy);
                                    msg!("BB[size1]: after cap -> {}", amount);
                                } else {
                                    msg!("BB[warn]: cap_tokens=0 (rtok=0), cannot buy");
                                }

                                // Guard rounding to zero → try minimum 1 atomic unit if affordable
                                if amount == 0 {
                                    match ctx.accounts.bonding_curve.buy_quote_checked(1) {
                                        Some(min_cost) => {
                                            msg!(
                                                "BB[fallback-atomic]: min_cost={} tpool={}",
                                                min_cost,
                                                tpool
                                            );
                                            if tpool >= min_cost && max_supply_buy >= 1 {
                                                amount = 1;
                                                msg!("BB[fallback-atomic]: amount set to 1");
                                            } else {
                                                msg!(
                                                    "BB[fallback-atomic]: insufficient tpool or cap<1"
                                                );
                                            }
                                        }
                                        None => msg!("BB[fallback-atomic]: unsafe quote(1)"),
                                    }
                                }

                                // Optional second fallback: try one *lot* if affordable and within caps
                                if amount == 0 && lot > 0 && max_supply_buy >= lot {
                                    match ctx.accounts.bonding_curve.buy_quote_checked(lot) {
                                        Some(lot_cost) => {
                                            msg!(
                                                "BB[fallback-lot]: lot={} cost={} tpool={}",
                                                lot,
                                                lot_cost,
                                                tpool
                                            );
                                            if tpool >= lot_cost {
                                                amount = lot.min(max_supply_buy);
                                                msg!("BB[fallback-lot]: amount set to {}", amount);
                                            } else {
                                                msg!(
                                                    "BB[fallback-lot]: insufficient tpool for lot"
                                                );
                                            }
                                        }
                                        None => msg!("BB[fallback-lot]: unsafe lot quote"),
                                    }
                                }

                                if amount > 0 {
                                    // Final quote & funds check
                                    match ctx.accounts.bonding_curve.buy_quote_checked(amount) {
                                        None => {
                                            msg!(
                                                "BB[skip]: unsafe quote for amount={} (vT={})",
                                                amount,
                                                ctx.accounts.bonding_curve.virtual_token_reserves
                                            );
                                        }
                                        Some(bb_cost) => {
                                            msg!(
                                                "BB[quote]: amount={} bb_cost={} tpool={}",
                                                amount,
                                                bb_cost,
                                                tpool
                                            );
                                            require!(
                                                tpool >= bb_cost,
                                                HorseFunError::InsufficientTreasuryFunds
                                            );

                                            // Snapshot pre-state for delta logs
                                            let vtr0 =
                                                ctx.accounts.bonding_curve.virtual_token_reserves;
                                            let rtr0 =
                                                ctx.accounts.bonding_curve.real_token_reserves;
                                            let vsr0 =
                                                ctx.accounts.bonding_curve.virtual_sol_reserves;
                                            let rsr0 = ctx.accounts.bonding_curve.real_sol_reserves;
                                            let tpool0 =
                                                ctx.accounts.bonding_curve.treasury_fee_pool;

                                            // Spend from treasury pool (book-keeping)
                                            ctx.accounts.bonding_curve.treasury_fee_pool =
                                                ctx.accounts.bonding_curve.treasury_fee_pool.saturating_sub(
                                                    bb_cost
                                                );

                                            // Apply the same state transition as a buy:
                                            ctx.accounts.bonding_curve.virtual_token_reserves =
                                                ctx.accounts.bonding_curve.virtual_token_reserves.saturating_sub(
                                                    amount
                                                );
                                            ctx.accounts.bonding_curve.virtual_sol_reserves =
                                                ctx.accounts.bonding_curve.virtual_sol_reserves.saturating_add(
                                                    bb_cost
                                                );

                                            ctx.accounts.bonding_curve.real_token_reserves =
                                                ctx.accounts.bonding_curve.real_token_reserves.saturating_sub(
                                                    amount
                                                );
                                            ctx.accounts.bonding_curve.real_sol_reserves =
                                                ctx.accounts.bonding_curve.real_sol_reserves.saturating_add(
                                                    bb_cost
                                                );

                                            // Intended burn sizing
                                            let mint_supply_before = ctx.accounts.mint.supply;

                                            burn_amount = amount; // burn the full buyback size
                                            let curve_ata_bal =
                                                ctx.accounts.associated_bonding_curve.amount;

                                            msg!("SHould burn:{}", burn_amount);
                                            msg!(
                                                "SBB[burn-plan]: burn_amount={} curve_ata_bal={} mint_supply_before={}",
                                                burn_amount,
                                                curve_ata_bal,
                                                ctx.accounts.mint.supply
                                            );

                                            // Safety guard
                                            let burn_ok =
                                                burn_amount > 0 && curve_ata_bal >= burn_amount;

                                            msg!(
                                                "SBB[burn-guard]: curve_ata>=burn?={} ({}>=?)",
                                                burn_ok,
                                                curve_ata_bal
                                            );

                                            let burn_ok =
                                                burn_amount > 0 && curve_ata_bal >= burn_amount;
                                            msg!(
                                                "SBB[burn-guard]: curve_ata>=intended?={} ({}>=?)",
                                                burn_ok,
                                                curve_ata_bal
                                            );

                                            // ---- BURN ENABLED ----
                                            if burn_ok {
                                                {
                                                    // Isolated scope to avoid borrow conflicts during CPI
                                                    helpers::burn_from_curve_ata_on_sell(
                                                        &ctx,
                                                        burn_amount
                                                    )?;
                                                }

                                                // IMPORTANT: do NOT subtract real_token_reserves again here.
                                                // We already reduced it by `amount` when applying the buyback math above.
                                                // Only update the cached total supply:

                                                // Also update circulating supply when tokens are burned
                                                ctx.accounts.bonding_curve.circulating_supply =
                                                    ctx.accounts.bonding_curve.circulating_supply.saturating_sub(
                                                        burn_amount
                                                    );
                                            }

                                            // track totals
                                            ctx.accounts.bonding_curve.total_treasury_spent =
                                                ctx.accounts.bonding_curve.total_treasury_spent.saturating_add(
                                                    bb_cost
                                                );

                                            let mut burned: u64 = 0;
                                            if burn_ok {
                                                // burn already executed above
                                                burned = burn_amount;
                                                ctx.accounts.bonding_curve.total_burned_supply =
                                                    ctx.accounts.bonding_curve.total_burned_supply.saturating_add(
                                                        burn_amount
                                                    );
                                            }

                                            let price_lpt = if amount > 0 {
                                                bb_cost.saturating_div(amount)
                                            } else {
                                                0
                                            };

                                            // Update buyback variables for TradeEvent at end of function
                                            is_buyback = true;
                                            burn_amount = burned;
                                            price_lamports_per_token = price_lpt;

                                            // Note: TradeEvent will be emitted at end of function

                                            // Delta log
                                            msg!(
                                                "BB[go]: amount={} cost={} new_tpool={}",
                                                amount,
                                                bb_cost,
                                                ctx.accounts.bonding_curve.treasury_fee_pool
                                            );
                                            msg!(
                                                "BB[delta]: Δvtok=-{} Δrtok=-{} Δvsol=+{} Δrsol=+{} Δtpool=-{}",
                                                vtr0.saturating_sub(
                                                    ctx.accounts.bonding_curve.virtual_token_reserves
                                                ),
                                                rtr0.saturating_sub(
                                                    ctx.accounts.bonding_curve.real_token_reserves
                                                ),
                                                ctx.accounts.bonding_curve.virtual_sol_reserves.saturating_sub(
                                                    vsr0
                                                ),
                                                ctx.accounts.bonding_curve.real_sol_reserves.saturating_sub(
                                                    rsr0
                                                ),
                                                tpool0.saturating_sub(
                                                    ctx.accounts.bonding_curve.treasury_fee_pool
                                                )
                                            );
                                        }
                                    }
                                } else {
                                    msg!("BB[skip]: amount=0 after sizing and fallbacks");
                                }
                            } else {
                                msg!(
                                    "BB[skip]: no trigger mkt={} thr={} bkt={}",
                                    market_lot,
                                    trigger_thr,
                                    backing_lot
                                );
                            }
                        }
                    }
                }
            } // Close the else block for burn limit check
        } else {
            msg!("Buybacks are disabled globally");
        }
        msg!(
            "SELL[payout-plan]: sol_output={} fee={} user_gets={}",
            sol_output,
            fee,
            sol_output.saturating_sub(fee)
        );

        // transfer the sol from the bonding curve to the user (excluding fee)
        helpers::transfer_sol_from_bonding_curve_to_user(&ctx, sol_output.saturating_sub(fee))?;

        // handle fee distribution from the retained amount
        helpers::transfer_sol_from_bonding_curve_to_fee_recipient(&mut ctx, fee)?;

        // Calculate fee splits for this trade to emit in event
        let (_, creator_fee_for_trade, _, _) = ctx.accounts.global.get_fee_splits(fee);

        // 🔧 Calculate if this user is a valid early bird
        let user_pos = ctx.accounts.holder_stats.entry_position;
        let is_early_bird =
            user_pos > 0 &&
            user_pos != u64::MAX &&
            user_pos <= ctx.accounts.global.early_bird_cutoff;

        // Emit final TradeEvent with all information (including possible buyback)
        let trade_event = TradeEvent {
            user: ctx.accounts.user.key(),
            sol_amount: sol_output,
            token_amount: amount,
            is_buy: false,
            mint: ctx.accounts.mint.key(),
            timestamp: Clock::get()?.unix_timestamp,
            virtual_sol_reserves: ctx.accounts.bonding_curve.virtual_sol_reserves,
            virtual_token_reserves: ctx.accounts.bonding_curve.virtual_token_reserves,
            circulating_supply: ctx.accounts.bonding_curve.circulating_supply,
            real_token_reserves: ctx.accounts.bonding_curve.real_token_reserves,
            real_sol_reserves: ctx.accounts.bonding_curve.real_sol_reserves,
            creator_fee_pool: ctx.accounts.bonding_curve.creator_fee_pool,
            treasury_fee_pool: ctx.accounts.bonding_curve.treasury_fee_pool,
            total_fees_accrued: ctx.accounts.bonding_curve.total_fees_accrued,
            total_treasury_fees_accrued: ctx.accounts.bonding_curve.total_treasury_fees_accrued,
            creator_fee_amount: creator_fee_for_trade, // Fee earned by creator from THIS trade
            fee_recipient: ctx.accounts.bonding_curve.creator_wallet, // Current creator (CTO-aware)
            is_buyback,
            burn_amount,
            price_lamports_per_token,
            total_burned_supply: ctx.accounts.bonding_curve.total_burned_supply,
            total_treasury_spent: ctx.accounts.bonding_curve.total_treasury_spent,
            early_bird_pool: ctx.accounts.bonding_curve.early_bird_pool,
            total_early_bird_fees_accrued: ctx.accounts.bonding_curve.total_early_bird_fees_accrued,
            user_position: ctx.accounts.holder_stats.entry_position,
            user_balance: ctx.accounts.holder_stats.current_balance,
            early_bird_cutoff: ctx.accounts.global.early_bird_cutoff,
            total_buyers: ctx.accounts.bonding_curve.total_buyers,
            early_bird_valid_count: ctx.accounts.bonding_curve.early_bird_valid_count,
            is_early_bird,
        };

        emit_cpi!(trade_event);
        emit!(trade_event);

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        require!(ctx.accounts.bonding_curve.complete, HorseFunError::BondingCurveNotComplete);
        require_keys_eq!(
            config_feature::withdraw_authority::ID,
            ctx.accounts.user.key(),
            HorseFunError::NotAuthorized
        );

        // Save creator fees - they can only be claimed by creator
        let creator_fees = ctx.accounts.bonding_curve.creator_fee_pool;

        // transfer the tokens from the bonding curve to the admin
        helpers::transfer_tokens_from_bonding_curve_to_admin(
            &ctx,
            ctx.accounts.associated_bonding_curve.amount
        )?;

        // transfer the sol from the bonding curve to the admin
        // Exclude creator fees from withdrawal
        helpers::transfer_sol_from_bonding_curve_to_admin(
            &ctx,
            ctx.accounts.bonding_curve.real_sol_reserves - creator_fees
        )?;

        // update the bonding curve parameters
        // Preserve creator fees
        ctx.accounts.bonding_curve.real_sol_reserves = creator_fees;
        ctx.accounts.bonding_curve.virtual_sol_reserves = 0;
        ctx.accounts.bonding_curve.real_token_reserves = 0;
        ctx.accounts.bonding_curve.virtual_token_reserves = 0;

        Ok(())
    }

    #[event]
    pub struct CreatorFeeClaimedEvent {
        pub mint: Pubkey,
        pub claimer: Pubkey,
        pub amount: u64,
        pub total_fees_accrued: u64,
        pub timestamp: i64,
    }

    #[event]
    pub struct StreamerIdentityCancelledEvent {
        pub user: Pubkey,
        pub streamer_id: String,
        pub timestamp: i64,
    }

    /// Register a streamer identity (only callable by platform authority)
    pub fn register_streamer_identity(
        ctx: Context<RegisterStreamerIdentity>,
        streamer_id: String
    ) -> Result<()> {
        msg!("Registering streamer identity:");
        msg!(" - Platform Authority: {}", ctx.accounts.platform_authority.key());
        msg!(" - User Wallet: {}", ctx.accounts.user.key());
        msg!(" - Streamer ID: {}", streamer_id);

        // Initialize StreamerIdRegistry
        let registry = &mut ctx.accounts.streamer_id_registry;
        registry.streamer_id = streamer_id.clone();
        registry.wallet = ctx.accounts.user.key();
        msg!(" - PDA Being Created: {}", ctx.accounts.streamer_identity.key());

        // Validate streamer_id format
        require!(
            !streamer_id.is_empty() && streamer_id.len() <= 50,
            HorseFunError::InvalidStreamerId
        );

        msg!("Streamer ID validation passed");

        // Initialize StreamerIdentity
        let streamer_identity = &mut ctx.accounts.streamer_identity;
        streamer_identity.wallet = ctx.accounts.user.key();
        streamer_identity.streamer_id = streamer_id.clone();
        streamer_identity.verified = true; // Set by platform

        msg!("StreamerIdentity account initialized:");
        msg!(" - Wallet: {}", streamer_identity.wallet);
        msg!(" - Streamer ID: {}", streamer_identity.streamer_id);
        msg!(" - Verified: {}", streamer_identity.verified);

        // Calculate and log PDA seeds
        let (expected_pda, bump) = Pubkey::find_program_address(
            &[b"streamer-identity", ctx.accounts.user.key().as_ref()],
            ctx.program_id
        );
        msg!("PDA Details:");
        msg!(" - Expected Address: {}", expected_pda);
        msg!(" - Bump Seed: {}", bump);
        msg!(" - Actual Address: {}", ctx.accounts.streamer_identity.key());

        // Emit registration event
        emit!(StreamerIdentityRegisteredEvent {
            user: ctx.accounts.user.key(),
            streamer_id,
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Streamer identity registration complete!");
        Ok(())
    }

    /// Claims creator's share of fees (can be claimed by creator or withdraw authority)
    /// Cancel a streamer identity registration (only callable by platform authority)
    pub fn cancel_streamer_identity(
        ctx: Context<CancelStreamerIdentity>,
        streamer_id: String
    ) -> Result<()> {
        msg!("Cancelling streamer identity:");
        msg!(" - Platform Authority: {}", ctx.accounts.platform_authority.key());
        msg!(" - User Wallet: {}", ctx.accounts.user.key());
        msg!(" - Streamer ID: {}", streamer_id);

        // Verify the streamer_id matches both accounts
        require!(
            ctx.accounts.streamer_identity.streamer_id == streamer_id &&
                ctx.accounts.streamer_id_registry.streamer_id == streamer_id,
            HorseFunError::InvalidStreamerId
        );

        // Verify the wallet matches in both accounts
        require!(
            ctx.accounts.streamer_identity.wallet == ctx.accounts.user.key() &&
                ctx.accounts.streamer_id_registry.wallet == ctx.accounts.user.key(),
            HorseFunError::UnauthorizedUser
        );

        msg!("Validation passed, closing accounts:");
        msg!(" - StreamerIdentity PDA: {}", ctx.accounts.streamer_identity.key());
        msg!(" - StreamerIdRegistry PDA: {}", ctx.accounts.streamer_id_registry.key());

        // The accounts will be automatically closed due to the close constraint
        // This will:
        // 1. Transfer the lamports to the platform authority
        // 2. Zero out the account data
        // 3. Mark the account as closed

        // Emit cancellation event
        emit!(StreamerIdentityCancelledEvent {
            user: ctx.accounts.user.key(),
            streamer_id,
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Streamer identity cancellation complete!");
        msg!(" - Rent returned to: {}", ctx.accounts.platform_authority.key());
        Ok(())
    }

    /// Reassigns the fee recipient for a token, overriding creator wallet and/or streamer ID
    /// Only callable by platform authority as a failsafe for community protection
    pub fn reassign_fee_recipient(
        ctx: Context<ReassignFeeRecipient>,
        new_recipient: Pubkey,
        new_streamer_id: Option<String>
    ) -> Result<()> {
        // Verify that the signer is the platform authority
        require_keys_eq!(
            config_feature::platform_authority::ID,
            ctx.accounts.platform_authority.key(),
            HorseFunError::NotAuthorized
        );

        msg!("Reassigning fee recipient:");
        msg!(" - Platform Authority: {}", ctx.accounts.platform_authority.key());
        msg!(" - Bonding Curve: {}", ctx.accounts.bonding_curve.key());
        msg!(" - Mint: {}", ctx.accounts.mint.key());
        msg!(" - Current Creator: {}", ctx.accounts.bonding_curve.creator_wallet);
        msg!(" - Current Streamer ID: {:?}", ctx.accounts.bonding_curve.creator_streamer_id);
        msg!(" - New Recipient: {}", new_recipient);
        msg!(" - New Streamer ID: {:?}", new_streamer_id);

        // Validate streamer_id format if provided
        if let Some(ref id) = new_streamer_id {
            require!(!id.is_empty() && id.len() <= 50, HorseFunError::InvalidStreamerId);
        }

        // Store original values for event
        let old_creator = ctx.accounts.bonding_curve.creator_wallet;
        let old_streamer_id = ctx.accounts.bonding_curve.creator_streamer_id.clone();

        // Update the bonding curve with new recipient info
        ctx.accounts.bonding_curve.creator_wallet = new_recipient;
        ctx.accounts.bonding_curve.creator_streamer_id = new_streamer_id.clone();

        // Emit the event
        // Emit CTO event with two separate struct constructions
        emit_cpi!(CtoEvent {
            mint: ctx.accounts.mint.key(),
            old_creator,
            old_streamer_id: old_streamer_id.clone(),
            new_creator: new_recipient,
            new_streamer_id: new_streamer_id.clone(),
            platform_authority: ctx.accounts.platform_authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        emit!(CtoEvent {
            mint: ctx.accounts.mint.key(),
            old_creator,
            old_streamer_id,
            new_creator: new_recipient,
            new_streamer_id,
            platform_authority: ctx.accounts.platform_authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn claim_creator_fees(ctx: Context<ClaimCreatorFees>) -> Result<()> {
        msg!("Attempting to claim creator fees:");
        msg!(" - Claimer Wallet: {}", ctx.accounts.user.key());
        msg!(" - Bonding Curve: {}", ctx.accounts.bonding_curve.key());
        msg!(" - Mint: {}", ctx.accounts.mint.key());
        msg!(" - Available Fees: {}", ctx.accounts.bonding_curve.creator_fee_pool);

        let fees = ctx.accounts.bonding_curve.creator_fee_pool;
        require!(fees > 0, HorseFunError::NoFeesToClaim);

        // Log verification path
        if ctx.accounts.user.key() == config_feature::withdraw_authority::ID {
            msg!("Claiming as withdraw authority");
        } else {
            match &ctx.accounts.bonding_curve.creator_streamer_id {
                Some(expected_id) => {
                    msg!("Token has streamer ID verification:");
                    msg!(" - Expected Streamer ID: {}", expected_id);

                    match &ctx.accounts.streamer_identity {
                        Some(identity_acc) => {
                            msg!(" - Provided Identity Account: {}", identity_acc.key());
                            // Log identity details after verification
                            if
                                let Ok(identity) = StreamerIdentity::try_deserialize(
                                    &mut &identity_acc.try_borrow_data()?[8..]
                                )
                            {
                                msg!(" - Identity Wallet: {}", identity.wallet);
                                msg!(" - Identity Streamer ID: {}", identity.streamer_id);
                                msg!(" - Identity Verified: {}", identity.verified);
                            } else {
                                msg!(" - Failed to deserialize identity account");
                            }
                        }
                        None => msg!(" - No streamer identity account provided"),
                    }
                }
                None => {
                    msg!("Token uses wallet-only verification:");
                    msg!(
                        " - Expected Creator Wallet: {}",
                        ctx.accounts.bonding_curve.creator_wallet
                    );
                }
            }
        }

        // Validate caller is either withdraw authority or verified creator
        ctx.accounts.validate()?;
        msg!("Creator validation passed");

        msg!("Initiating fee transfer:");
        msg!(" - From Bonding Curve: {}", ctx.accounts.bonding_curve.key());
        msg!(" - To Wallet: {}", ctx.accounts.user.key());
        msg!(" - Amount: {} lamports", fees);

        // Transfer fees to claimer
        // After `ctx.accounts.validate()?;`
        let fees = ctx.accounts.bonding_curve.creator_fee_pool;
        require!(fees > 0, HorseFunError::NoFeesToClaim);
        let mint_key = ctx.accounts.mint.key();
        let bump_bytes = [ctx.bumps.bonding_curve];
        let seeds = helpers::curve_seeds(&mint_key, &bump_bytes);

        helpers::pda_transfer_lamports(
            &ctx.accounts.bonding_curve.to_account_info(),
            &ctx.accounts.user.to_account_info(),
            fees
        )?;

        msg!("Fee transfer successful");

        // Emit claim event
        emit!(CreatorFeeClaimedEvent {
            mint: ctx.accounts.mint.key(),
            claimer: ctx.accounts.user.key(),
            amount: fees,
            total_fees_accrued: ctx.accounts.bonding_curve.total_fees_accrued,
            timestamp: Clock::get()?.unix_timestamp,
        });

        // Reset fee pool
        ctx.accounts.bonding_curve.creator_fee_pool = 0;
        msg!("Fee pool reset to 0");

        Ok(())
    }

    /// Send a message with a donation to a creator
    pub fn send_message(ctx: Context<SendMessage>, message: String, amount: u64) -> Result<()> {
        // Validate message length
        require!(message.len() <= 200, HorseFunError::MessageTooLong);
        require!(amount > 0, HorseFunError::InsufficientDonationAmount);

        // Transfer SOL from user to message list account
        transfer(
            CpiContext::new(ctx.accounts.system_program.to_account_info(), Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.message_list.to_account_info(),
            }),
            amount
        )?;

        // Create new message
        let message_data = Message {
            sender: ctx.accounts.user.key(),
            amount,
            message,
            timestamp: Clock::get()?.unix_timestamp,
        };

        // Update message list
        ctx.accounts.message_list.messages.push(message_data.clone());
        ctx.accounts.message_list.total_received += amount;
        ctx.accounts.message_list.unclaimed_amount += amount;

        // Emit event
        emit!(MessageSentEvent {
            mint: ctx.accounts.mint.key(),
            sender: ctx.accounts.user.key(),
            amount,
            message: message_data.message,
            timestamp: message_data.timestamp,
        });

        Ok(())
    }

    /// Claim all unclaimed donations for a creator
    pub fn claim_donations(ctx: Context<ClaimDonations>) -> Result<()> {
        let unclaimed = ctx.accounts.message_list.unclaimed_amount;
        require!(unclaimed > 0, HorseFunError::NoDonationsToClaim);

        // Validate caller is either withdraw authority or verified creator
        ctx.accounts.validate()?;

        let amount = ctx.accounts.message_list.unclaimed_amount;
        require!(amount > 0, HorseFunError::NoDonationsToClaim);

        let mint_key = ctx.accounts.mint.key();
        let bump_bytes = [ctx.bumps.message_list];
        let seeds = helpers::msglist_seeds(&mint_key, &bump_bytes);
        helpers::pda_transfer_lamports(
            &ctx.accounts.message_list.to_account_info(),
            &ctx.accounts.user.to_account_info(),
            amount
        )?;
        // Reset unclaimed amount while preserving total_received
        ctx.accounts.message_list.unclaimed_amount = 0;

        // Emit event with full claimed amount
        emit!(DonationClaimedEvent {
            mint: ctx.accounts.mint.key(),
            creator: ctx.accounts.user.key(),
            amount: unclaimed,
            timestamp: Clock::get()?.unix_timestamp,
            remaining_unclaimed: 0,
        });

        Ok(())
    }

    /// Claim early bird rewards for eligible holders
    pub fn claim_early_bird_rewards(ctx: Context<ClaimEarlyBirdRewards>) -> Result<()> {
        let holder_stats = &mut ctx.accounts.holder_stats;
        let bonding_curve = &mut ctx.accounts.bonding_curve;
        let global = &ctx.accounts.global;

        // Check if early bird rewards are enabled
        require!(global.early_bird_enabled, HorseFunError::EarlyBirdDisabled);

        // ⭐ NEW: Require bonding curve to be complete before claiming
        require!(bonding_curve.complete, HorseFunError::CurveNotComplete);

        // Check if user is eligible (within first X buyers AND not revoked)
        // entry_position == 0: never bought before
        // entry_position == u64::MAX: permanently revoked (sold)
        // entry_position > 0 && <= cutoff: eligible early bird
        require!(
            holder_stats.entry_position > 0 &&
                holder_stats.entry_position != u64::MAX &&
                holder_stats.entry_position <= global.early_bird_cutoff,
            HorseFunError::NotEarlyBird
        );

        // ⭐ SECURITY: Prevent double claiming
        // fees_claimed > 0 means they already claimed their Early Bird reward
        require!(holder_stats.fees_claimed == 0, HorseFunError::AlreadyClaimedEarlyBird);

        // Calculate claimable amount using cached equal share
        // The share was calculated once when curve completed: pool / valid_count
        // This ensures ALL early birds get EXACTLY the same amount
        let share = bonding_curve.early_bird_share_per_seat;
        require!(share > 0, HorseFunError::NoRewardsToClaim);

        msg!("🐦 Claiming Early Bird Rewards:");
        msg!(" - User Position: #{}", holder_stats.entry_position);
        msg!(" - Valid Early Bird Seats at Completion: {}", bonding_curve.early_bird_valid_count);
        msg!(" - Equal Share Per Seat (cached): {} lamports", share);
        msg!(" - Pool Before Claim: {} lamports", bonding_curve.early_bird_pool);

        // Transfer SOL from bonding curve to user using PDA transfer
        helpers::pda_transfer_lamports(
            &bonding_curve.to_account_info(),
            &ctx.accounts.user.to_account_info(),
            share
        )?;

        // Update tracking
        bonding_curve.early_bird_pool -= share;
        holder_stats.fees_claimed += share;

        emit!(EarlyBirdClaimed {
            user: ctx.accounts.user.key(),
            mint: ctx.accounts.mint.key(),
            amount: share,
            position: holder_stats.entry_position,
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("✅ Early Bird Rewards claimed successfully!");

        Ok(())
    }
}

mod helpers {
    use super::*;

    /// Revoke Early Bird status permanently when user sells
    /// Uses u64::MAX as sentinel value to prevent re-qualification on future buys
    /// Also decrements the valid early bird count
    pub fn revoke_early_bird_status(
        holder_stats: &mut HolderStats,
        bonding_curve: &mut BondingCurve,
        global: &Global
    ) {
        if holder_stats.entry_position > 0 && holder_stats.entry_position != u64::MAX {
            // Check if this position was within the early bird cutoff
            if holder_stats.entry_position <= global.early_bird_cutoff {
                // Decrement valid count since this early bird seat is now revoked
                bonding_curve.early_bird_valid_count =
                    bonding_curve.early_bird_valid_count.saturating_sub(1);

                msg!(
                    "💔 Early Bird status revoked! Position #{} forfeited FOREVER. Valid seats remaining: {}",
                    holder_stats.entry_position,
                    bonding_curve.early_bird_valid_count
                );
            }

            holder_stats.entry_position = u64::MAX; // MAX = permanently revoked
        }
    }

    #[inline]
    pub fn pda_transfer_lamports(
        from: &AccountInfo,
        to: &AccountInfo,
        lamports: u64
    ) -> Result<()> {
        // Must be program-owned
        require_keys_eq!(*from.owner, crate::ID, HorseFunError::NotAuthorized);
        require!(lamports > 0, HorseFunError::InsufficientDonationAmount);

        // -- debit: single scoped mutable borrow --
        {
            let mut from_lams = from.try_borrow_mut_lamports()?; // RefMut<&mut u64>
            let available: u64 = **from_lams; // read value
            require!(available >= lamports, HorseFunError::InsufficientTreasuryFunds);
            **from_lams = available.saturating_sub(lamports); // write value
        } // drop borrow

        // -- credit: single scoped mutable borrow --
        {
            let mut to_lams = to.try_borrow_mut_lamports()?; // RefMut<&mut u64>
            let cur: u64 = **to_lams; // read value
            **to_lams = cur.saturating_add(lamports); // write value
        } // drop borrow

        Ok(())
    }

    pub fn curve_seeds<'a>(mint: &'a Pubkey, bump: &'a [u8; 1]) -> [&'a [u8]; 3] {
        [b"bonding-curve", mint.as_ref(), bump.as_ref()]
    }

    pub fn msglist_seeds<'a>(mint: &'a Pubkey, bump: &'a [u8; 1]) -> [&'a [u8]; 3] {
        [b"message-list", mint.as_ref(), bump.as_ref()]
    }

    pub fn transfer_tokens_from_bonding_curve_to_admin(
        ctx: &Context<Withdraw>,
        token_amount: u64
    ) -> Result<()> {
        let mint_key = ctx.accounts.mint.key();
        let authority_seed = &[
            b"bonding-curve".as_ref(),
            mint_key.as_ref(),
            &[ctx.bumps.bonding_curve],
        ];
        let seeds = [authority_seed.as_slice()];

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.associated_bonding_curve.to_account_info(),
                    to: ctx.accounts.associated_user.to_account_info(),
                    authority: ctx.accounts.bonding_curve.to_account_info(),
                },
                &seeds
            ),
            token_amount
        )
    }

    pub fn transfer_sol_from_bonding_curve_to_admin(
        ctx: &Context<Withdraw>,
        sol_amount: u64
    ) -> Result<()> {
        helpers::pda_transfer_lamports(
            &ctx.accounts.bonding_curve.to_account_info(),
            &ctx.accounts.user.to_account_info(),
            sol_amount
        )
    }

    pub fn transfer_sol_from_bonding_curve_to_user(
        ctx: &Context<Sell>,
        sol_amount: u64
    ) -> Result<()> {
        let mint_key = ctx.accounts.mint.key();
        let bump_bytes = [ctx.bumps.bonding_curve];
        let seeds = curve_seeds(&mint_key, &bump_bytes);
        helpers::pda_transfer_lamports(
            &ctx.accounts.bonding_curve.to_account_info(),
            &ctx.accounts.user.to_account_info(),
            sol_amount
        )
    }

    pub fn transfer_sol_from_bonding_curve_to_fee_recipient(
        ctx: &mut Context<Sell>,
        sol_amount: u64
    ) -> Result<()> {
        // Validate recipient
        require_keys_eq!(
            ctx.accounts.global.fee_recipient,
            ctx.accounts.fee_recipient.key(),
            HorseFunError::NotAuthorized
        );

        // Split fees (4-way split now)
        let (platform_fee, creator_fee, treasury_fee, early_bird_fee) =
            ctx.accounts.global.get_fee_splits(sol_amount);

        // Book-keep fee pools (creator/treasury/early_bird stay on curve)
        ctx.accounts.bonding_curve.creator_fee_pool += creator_fee;
        ctx.accounts.bonding_curve.treasury_fee_pool += treasury_fee;
        ctx.accounts.bonding_curve.early_bird_pool += early_bird_fee;
        ctx.accounts.bonding_curve.total_fees_accrued += creator_fee;
        ctx.accounts.bonding_curve.total_treasury_fees_accrued += treasury_fee;
        ctx.accounts.bonding_curve.total_early_bird_fees_accrued += early_bird_fee;

        // Move platform fee from curve PDA → platform using invoke_signed
        if platform_fee > 0 {
            helpers::pda_transfer_lamports(
                &ctx.accounts.bonding_curve.to_account_info(),
                &ctx.accounts.fee_recipient.to_account_info(),
                platform_fee
            )?;
        }

        Ok(())
    }

    pub fn transfer_tokens_from_user_to_bonding_curve(
        ctx: &Context<Sell>,
        token_amount: u64
    ) -> Result<()> {
        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), token::Transfer {
                from: ctx.accounts.associated_user.to_account_info(),
                to: ctx.accounts.associated_bonding_curve.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            }),
            token_amount
        )
    }

    pub fn transfer_tokens_from_bonding_curve_to_user(
        ctx: &Context<Buy>,
        token_amount: u64
    ) -> Result<()> {
        let mint_key = ctx.accounts.mint.key();
        let authority_seed = &[
            b"bonding-curve".as_ref(),
            mint_key.as_ref(),
            &[ctx.bumps.bonding_curve],
        ];
        let seeds = [authority_seed.as_slice()];

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.associated_bonding_curve.to_account_info(),
                    to: ctx.accounts.associated_user.to_account_info(),
                    authority: ctx.accounts.bonding_curve.to_account_info(),
                },
                &seeds
            ),
            token_amount
        )
    }

    pub fn burn_from_curve_ata_on_buy(ctx: &Context<Buy>, token_amount: u64) -> Result<()> {
        let mint_key = ctx.accounts.mint.key();
        let authority_seed = &[
            b"bonding-curve".as_ref(),
            mint_key.as_ref(),
            &[ctx.bumps.bonding_curve],
        ];
        let seeds = [authority_seed.as_slice()];

        token::burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Burn {
                    mint: ctx.accounts.mint.to_account_info(),
                    from: ctx.accounts.associated_bonding_curve.to_account_info(),
                    authority: ctx.accounts.bonding_curve.to_account_info(),
                },
                &seeds
            ),
            token_amount
        )
    }

    pub fn burn_from_curve_ata_on_sell(ctx: &Context<Sell>, token_amount: u64) -> Result<()> {
        let mint_key = ctx.accounts.mint.key();
        let authority_seed = &[
            b"bonding-curve".as_ref(),
            mint_key.as_ref(),
            &[ctx.bumps.bonding_curve],
        ];
        let seeds = [authority_seed.as_slice()];

        token::burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Burn {
                    mint: ctx.accounts.mint.to_account_info(),
                    from: ctx.accounts.associated_bonding_curve.to_account_info(),
                    authority: ctx.accounts.bonding_curve.to_account_info(),
                },
                &seeds
            ),
            token_amount
        )
    }

    pub fn transfer_sol_from_user_to_bonding_curve(
        ctx: &Context<Buy>,
        sol_amount: u64
    ) -> Result<()> {
        msg!("Transferring {} SOL from user to bonding curve", sol_amount);
        // transfer sol to bonding curve (excluding fees)
        transfer(
            CpiContext::new(ctx.accounts.system_program.to_account_info(), Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.bonding_curve.to_account_info(),
            }),
            sol_amount
        )
    }

    pub fn transfer_sol_from_user_to_fee_recipient(
        ctx: &mut Context<Buy>,
        fee_amount: u64
    ) -> Result<()> {
        // Check fee recipient matches global state
        require_keys_eq!(
            ctx.accounts.global.fee_recipient,
            ctx.accounts.fee_recipient.key(),
            HorseFunError::NotAuthorized
        );

        // Split fees according to global parameters (4-way split)
        let (platform_fee, creator_fee, treasury_fee, early_bird_fee) =
            ctx.accounts.global.get_fee_splits(fee_amount);

        // Update fee pools - these are tracked separately from reserves
        ctx.accounts.bonding_curve.creator_fee_pool += creator_fee;
        ctx.accounts.bonding_curve.treasury_fee_pool += treasury_fee;
        ctx.accounts.bonding_curve.early_bird_pool += early_bird_fee;
        ctx.accounts.bonding_curve.total_fees_accrued += creator_fee;
        ctx.accounts.bonding_curve.total_treasury_fees_accrued += treasury_fee;
        ctx.accounts.bonding_curve.total_early_bird_fees_accrued += early_bird_fee;

        // Transfer platform fee directly to fee recipient
        transfer(
            CpiContext::new(ctx.accounts.system_program.to_account_info(), Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.fee_recipient.to_account_info(),
            }),
            platform_fee
        )?;

        // Transfer creator, treasury, and early bird fees to bonding curve account
        if creator_fee + treasury_fee + early_bird_fee > 0 {
            transfer(
                CpiContext::new(ctx.accounts.system_program.to_account_info(), Transfer {
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.bonding_curve.to_account_info(),
                }),
                creator_fee + treasury_fee + early_bird_fee
            )?;
        }

        Ok(())
    }

    pub fn mint_to_bonding_curve<'info>(ctx: &Context<Create>) -> Result<()> {
        let authority_seed = &[b"mint-authority".as_ref(), &[ctx.bumps.mint_authority]];
        let seeds = [authority_seed.as_slice()];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.associated_bonding_curve.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &seeds
        );

        token::mint_to(cpi_ctx, ctx.accounts.global.token_total_supply)
    }

    pub fn set_metadata<'info>(
        ctx: &Context<Create>,
        name: String,
        symbol: String,
        uri: String
    ) -> Result<()> {
        // set the metadata for the token
        let data = DataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        let authority_seed = &[b"mint-authority".as_ref(), &[ctx.bumps.mint_authority]];
        let seeds = [authority_seed.as_slice()];

        let metadata_ctx = CreateMetadataAccountsV3 {
            metadata: ctx.accounts.metadata.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            mint_authority: ctx.accounts.mint_authority.to_account_info(),
            payer: ctx.accounts.user.to_account_info(),
            update_authority: ctx.accounts.mint_authority.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.mpl_token_metadata.to_account_info(),
            metadata_ctx,
            &seeds
        );

        metadata::create_metadata_accounts_v3(cpi_ctx, data, false, true, None)
    }

    pub fn revoke_mint_authority(ctx: &Context<Create>) -> Result<()> {
        // renounce the mint authority
        let renounce_accounts = SetAuthority {
            account_or_mint: ctx.accounts.mint.to_account_info(),
            current_authority: ctx.accounts.mint_authority.to_account_info(),
        };

        let authority_seed = &[b"mint-authority".as_ref(), &[ctx.bumps.mint_authority]];
        let seeds = [authority_seed.as_slice()];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            renounce_accounts,
            &seeds
        );

        token::set_authority(cpi_ctx, AuthorityType::MintTokens, None)
    }
}

#[derive(Accounts)]
pub struct ClaimCreatorFees<'info> {
    pub mint: Account<'info, Mint>,
    #[account(mut, seeds = [b"bonding-curve", mint.key().as_ref()], bump)]
    pub bonding_curve: Account<'info, BondingCurve>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: We validate in custom logic
    pub streamer_identity: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimCreatorFees<'info> {
    pub fn validate(&self) -> Result<()> {
        // First check if it's the withdraw authority (platform failsafe)
        if self.user.key() == config_feature::withdraw_authority::ID {
            msg!("Authorized: Withdraw authority");
            return Ok(());
        }

        // Check if token was created with a streamer ID
        if let Some(token_streamer_id) = self.bonding_curve.creator_streamer_id.as_ref() {
            msg!("Token has streamer ID: {}", token_streamer_id);

            // When streamer_id exists, ONLY a verified streamer can claim
            let streamer_account_info = self.streamer_identity.as_ref().ok_or_else(|| {
                msg!("Unauthorized: No streamer identity provided for token with streamer ID");
                HorseFunError::UnauthorizedCreator
            })?;

            msg!(
                "Attempting to deserialize streamer identity from account: {}",
                streamer_account_info.key()
            );

            // Try to deserialize the streamer identity
            let streamer_identity = StreamerIdentity::try_deserialize(
                &mut &streamer_account_info.try_borrow_data()?[..]
            ).map_err(|_| {
                msg!("Failed to deserialize streamer identity account");
                HorseFunError::UnauthorizedCreator
            })?;

            // Verify the identity matches exactly
            if
                streamer_identity.wallet == self.user.key() &&
                streamer_identity.verified &&
                streamer_identity.streamer_id == *token_streamer_id
            {
                msg!("Authorized: Verified streamer identity");
                return Ok(());
            }

            msg!("Unauthorized: Invalid or unverified streamer identity");
            return Err(HorseFunError::UnauthorizedCreator.into());
        } else {
            msg!("Token has no streamer ID, checking creator wallet");
            // No streamer ID, ONLY creator wallet can claim
            require!(
                self.user.key() == self.bonding_curve.creator_wallet,
                HorseFunError::UnauthorizedCreator
            );
            msg!("Authorized: Original creator wallet");
            return Ok(());
        }
    }
}

#[account]
pub struct StreamerIdentity {
    pub wallet: Pubkey,
    pub streamer_id: String,
    pub verified: bool,
}

#[account]
pub struct StreamerIdRegistry {
    pub streamer_id: String,
    pub wallet: Pubkey,
}

impl StreamerIdentity {
    pub const SIZE: usize = 8 + 32 + 50 + 1; // 50 chars for streamer_id
}

impl StreamerIdRegistry {
    pub const SIZE: usize = 8 + 50 + 32; // discriminator + streamer_id + wallet
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub struct BuybackParams {
    pub backing_mult_bps: u16, // e.g. 11000 = 1.10× backing
    pub ema_drop_bps: u16, // e.g. 9500  = 0.95× EMA
    pub ema_alpha_bps: u16, // e.g. 2000  = 20% smoothing
    pub spend_bps: u16, // e.g. 2000  = 20% treasury per trigger
    pub max_supply_bps: u16, // e.g. 1000  = 10% on-curve cap
    pub min_backing_lamports: u64, // floor to avoid dust-trigger
    pub max_burn_percentage_bps: u16, // e.g. 2500 = 25% max total burn of total supply
}

#[event]
pub struct StreamerIdentityRegisteredEvent {
    pub user: Pubkey,
    pub streamer_id: String,
    pub timestamp: i64,
}

#[event_cpi]
#[derive(Accounts)]
pub struct ReassignFeeRecipient<'info> {
    /// The platform authority that can reassign fee recipients
    #[account(
        mut,
        constraint = platform_authority.key() == config_feature::platform_authority::ID
    )]
    pub platform_authority: Signer<'info>,

    /// The mint for the token
    pub mint: Account<'info, Mint>,

    /// The bonding curve to update
    #[account(
        mut, 
        seeds = [b"bonding-curve", mint.key().as_ref()], 
        bump
    )]
    pub bonding_curve: Account<'info, BondingCurve>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(streamer_id: String)]
pub struct RegisterStreamerIdentity<'info> {
    /// The platform authority that can register Streamer identities
    #[account(
        mut,
        constraint = platform_authority.key() == config_feature::platform_authority::ID
    )]
    pub platform_authority: Signer<'info>,

    /// The user's wallet to link with Streamer
    /// CHECK: This is the user who will own the StreamerIdentity
    pub user: UncheckedAccount<'info>,

    /// The StreamerIdentity account to create
    #[account(
        init,
        payer = platform_authority,
        space = StreamerIdentity::SIZE,
        seeds = [b"streamer-identity", user.key().as_ref()],
        bump
    )]
    pub streamer_identity: Account<'info, StreamerIdentity>,

    /// Registry to ensure streamer_id uniqueness
    #[account(
        init,
        payer = platform_authority,
        space = StreamerIdRegistry::SIZE,
        seeds = [b"streamer-id-registry", streamer_id.as_bytes()],
        bump
    )]
    pub streamer_id_registry: Account<'info, StreamerIdRegistry>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(streamer_id: String)]
pub struct CancelStreamerIdentity<'info> {
    /// The platform authority that can cancel Streamer identities
    #[account(
        mut,
        constraint = platform_authority.key() == config_feature::platform_authority::ID
    )]
    pub platform_authority: Signer<'info>,

    /// The user's wallet to unlink
    /// CHECK: This is the user whose StreamerIdentity is being cancelled
    pub user: UncheckedAccount<'info>,

    /// The StreamerIdentity account to close
    #[account(
        mut,
        close = platform_authority,
        seeds = [b"streamer-identity", user.key().as_ref()],
        bump
    )]
    pub streamer_identity: Account<'info, StreamerIdentity>,

    /// Registry to ensure streamer_id uniqueness
    #[account(
        mut,
        close = platform_authority,
        seeds = [b"streamer-id-registry", streamer_id.as_bytes()],
        bump
    )]
    pub streamer_id_registry: Account<'info, StreamerIdRegistry>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum HorseFunError {
    #[msg("Fee shares must add up to 100% (10000 basis points)")]
    InvalidFeeShares,
    #[msg("Invalid or missing streamer ID")]
    InvalidStreamerId,
    #[msg("Streamer ID already registered to another wallet")]
    StreamerIdAlreadyRegistered,
    #[msg("Unauthorized creator")]
    UnauthorizedCreator,
    #[msg("No fees available to claim")]
    NoFeesToClaim,
    #[msg("The given account is not authorized to execute this instruction.")]
    NotAuthorized,
    #[msg("The program is already initialized.")]
    AlreadyInitialized,
    #[msg("slippage: Too much SOL required to buy the given amount of tokens.")]
    TooMuchSolRequired,
    #[msg("slippage: Too little SOL received to sell the given amount of tokens.")]
    TooLittleSolReceived,
    #[msg("The mint does not match the bonding curve.")]
    MintDoesNotMatchBondingCurve,
    #[msg("The bonding curve has completed and liquidity migrated to raydium.")]
    BondingCurveComplete,
    #[msg("The bonding curve has not completed.")]
    BondingCurveNotComplete,
    #[msg("The program is not initialized.")]
    NotInitialized,
    #[msg("Unauthorized user")]
    UnauthorizedUser,
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[msg("Insufficient funds in treasury for buyback")]
    InsufficientTreasuryFunds,
    #[msg("Message exceeds maximum length of 200 characters")]
    MessageTooLong,
    #[msg("No donations available to claim")]
    NoDonationsToClaim,
    #[msg("Insufficient donation amount")]
    InsufficientDonationAmount,
    #[msg("Only the creator or withdraw authority can claim donations")]
    UnauthorizedDonationClaim,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Early bird rewards are disabled")]
    EarlyBirdDisabled,
    #[msg("User is not an early bird (not in first X buyers)")]
    NotEarlyBird,
    #[msg("No rewards available to claim")]
    NoRewardsToClaim,
    #[msg("No early birds registered yet")]
    NoEarlyBirds,
    #[msg("Buy amount below minimum required for Early Bird eligibility")]
    BuyAmountTooSmall,
    #[msg("Bonding curve must be complete before claiming Early Bird rewards")]
    CurveNotComplete,
    #[msg("Early Bird rewards already claimed - can only claim once")]
    AlreadyClaimedEarlyBird,
    #[msg("Arithmetic overflow or underflow")]
    ArithmeticOverflow,
}

#[account]
pub struct Global {
    pub initialized: bool,
    pub authority: Pubkey,
    pub fee_recipient: Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
    pub creator_fee_share: u64, // Percentage of fees that go to creator
    pub platform_fee_share: u64, // Percentage of fees that go to platform
    pub treasury_fee_share: u64, // Percentage of fees that go to treasury
    pub early_bird_fee_share: u64, // Percentage of fees that go to early bird pool
    pub buybacks_enabled: bool, // Toggle for buyback functionality
    pub buyback_params: BuybackParams,
    pub early_bird_enabled: bool, // Toggle for early bird rewards
    pub early_bird_cutoff: u64, // First X buyers get early bird rewards (e.g., 50)
    pub early_bird_min_buy_sol: u64, // Minimum SOL amount to qualify for early bird (in lamports)
}

impl Global {
    // Updated size calculation: added early_bird_fee_share (8), early_bird_enabled (1), early_bird_cutoff (8), early_bird_min_buy_sol (8)
    pub const SIZE: usize =
        8 + 32 + 1 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 18 + 1 + 8 + 8;

    pub fn get_fee(&self, amount: u64) -> u64 {
        let fee = ((amount as u128) * (self.fee_basis_points as u128)) / 10_000;
        return fee as u64;
    }

    pub fn get_fee_splits(&self, total_fee: u64) -> (u64, u64, u64, u64) {
        let platform_fee = ((total_fee as u128) * (self.platform_fee_share as u128)) / 10_000;
        let creator_fee = ((total_fee as u128) * (self.creator_fee_share as u128)) / 10_000;
        let treasury_fee = ((total_fee as u128) * (self.treasury_fee_share as u128)) / 10_000;
        let early_bird_fee = ((total_fee as u128) * (self.early_bird_fee_share as u128)) / 10_000;
        (platform_fee as u64, creator_fee as u64, treasury_fee as u64, early_bird_fee as u64)
    }
}

#[account]
pub struct BondingCurve {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub circulating_supply: u64, // Track circulating supply separate from total supply
    pub complete: bool,
    // Creator info and fees
    pub creator_wallet: Pubkey,
    pub creator_streamer_id: Option<String>, // Optional streamer ID for verification
    pub creator_fee_pool: u64, // Creator's share of fees
    pub treasury_fee_pool: u64, // Treasury's share of fees
    pub total_fees_accrued: u64, // Total creator fees accrued
    pub total_treasury_fees_accrued: u64, // Total treasury fees accrued
    pub ema_lot_price: u64,

    // Running totals for analytics
    pub total_burned_supply: u64, // how much supply we have burned (ever)
    pub total_treasury_spent: u64, // how many lamports spent on buybacks (ever)

    // Early Bird Rewards
    pub early_bird_pool: u64, // Accumulated SOL for early bird rewards
    pub total_buyers: u64, // Total number of unique buyers (for position tracking)
    pub total_early_bird_fees_accrued: u64, // Total historical early bird fees
    pub early_bird_valid_count: u64, // Number of valid (non-revoked) early bird seats
    pub early_bird_share_per_seat: u64, // Equal share amount calculated when curve completes (pool / valid_count)
}

#[account]
pub struct HolderStats {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub current_balance: u64,
    pub fees_claimed: u64,
    pub entry_position: u64, // Position in line (1 = first buyer, 2 = second, etc.)
    pub total_volume: u64, // Lifetime trading volume for analytics
}

impl HolderStats {
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 8 + 8 + 8;

    pub fn update_stats(&mut self, _clock: &Clock) -> Result<()> {
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Message {
    pub sender: Pubkey,
    pub amount: u64,
    pub message: String,
    pub timestamp: i64,
}

#[account]
pub struct MessageList {
    pub mint: Pubkey, // Associated token mint
    pub total_received: u64, // Total historical donations
    pub unclaimed_amount: u64, // Current unclaimed balance
    pub messages: Vec<Message>, // All messages received
}

impl MessageList {
    pub const SIZE: usize =
        8 + // Discriminator
        32 + // mint
        8 + // total_received
        8 + // unclaimed_amount
        4 + // Vec length
        10 * (32 + 8 + (4 + 200) + 8); // 10 messages capacity
}

impl BondingCurve {
    /// Return the price to buy `amount` atomic units; safe guard.
    pub fn buy_quote_checked(&self, amount: u64) -> Option<u64> {
        if amount == 0 {
            return Some(0);
        }
        let v_s = self.virtual_sol_reserves as u128;
        let v_t = self.virtual_token_reserves as u128;
        let a = amount as u128;
        if a >= v_t {
            return None;
        } // would underflow
        let num = a.checked_mul(v_s)?;
        let den = v_t.checked_sub(a)?;
        let q = num / den;
        Some((q as u64).saturating_add(1))
    }

    /// Backing per *lot* using treasury_pool and current circulating supply.
    /// lot is in atomic units (e.g., 1 token = 10^decimals).
    pub fn backing_per_lot_with_treasury(&self, lot: u64, treasury_pool: u64) -> u64 {
        if self.complete || lot == 0 {
            return 0;
        }
        // Use circulating supply directly
        if self.circulating_supply == 0 {
            return 0;
        }
        // lamports per lot := (treasury_pool / circ) * lot, but do it in u128
        let tp = treasury_pool as u128;
        let l = lot as u128;
        let c = self.circulating_supply as u128;
        (tp.saturating_mul(l) / c) as u64
    }

    /// Update EMA of lot price. Alpha in basis points (e.g., 2_000 for 20%).
    pub fn update_ema_lot_price(&mut self, new_price: u64, alpha_bps: u64) {
        // Initialize EMA on first update
        if self.ema_lot_price == 0 {
            self.ema_lot_price = new_price;
            return;
        }
        let a = alpha_bps.min(10_000) as u128;
        let na = 10_000u128 - a;
        let old = self.ema_lot_price as u128;
        let new = new_price as u128;
        self.ema_lot_price = ((a * new + na * old) / 10_000) as u64;
    }

    /// Invert the CPMM to size tokens for a SOL budget:
    /// budget = Δt * vS / (vT - Δt)  => Δt = budget * vT / (vS + budget)
    pub fn tokens_for_budget(&self, budget: u64) -> u64 {
        if budget == 0 {
            return 0;
        }
        let v_s = self.virtual_sol_reserves as u128;
        let v_t = self.virtual_token_reserves as u128;
        let b = budget as u128;
        let den = v_s.saturating_add(b);
        if den == 0 {
            return 0;
        }
        let num = b.saturating_mul(v_t);
        let mut dt = (num / den) as u64;
        // must be strictly < v_t to avoid division by zero in quotes
        let max_buy = self.virtual_token_reserves.saturating_sub(1);
        if dt > max_buy {
            dt = max_buy;
        }
        dt
    }

    pub fn backing_per_token(&self) -> u64 {
        if self.complete {
            return 0; // curve finished, funds migrate
        }

        let circulating_supply = self.token_total_supply.saturating_sub(self.real_token_reserves);

        if circulating_supply == 0 {
            return 0;
        }

        // Include both real_sol_reserves and treasury_fee_pool for total backing
        let total_backing = self.real_sol_reserves.saturating_add(self.treasury_fee_pool);
        total_backing / circulating_supply
    }

    pub fn buy_quote(&self, amount: u128) -> u64 {
        let virtual_sol_reserves = self.virtual_sol_reserves as u128;
        let virtual_token_reserves = self.virtual_token_reserves as u128;
        let sol_cost: u64 = ((amount * virtual_sol_reserves) /
            (virtual_token_reserves - amount)) as u64;

        return sol_cost + 1; // always round up
    }

    pub fn sell_quote(&self, amount: u128) -> u64 {
        let virtual_sol_reserves = self.virtual_sol_reserves as u128;
        let virtual_token_reserves = self.virtual_token_reserves as u128;
        let sol_output: u64 = ((amount * virtual_sol_reserves) /
            (virtual_token_reserves + amount)) as u64;

        return sol_output;
    }
}

impl BondingCurve {
    pub const SIZE: usize =
        8 + // discriminator
        8 + // virtual_token_reserves
        8 + // virtual_sol_reserves
        8 + // real_token_reserves
        8 + // real_sol_reserves
        8 + // token_total_supply
        8 + // circulating_supply
        1 + // complete
        32 + // creator_wallet
        (4 + 50) + // creator_streamer_id (Option<String>)
        8 + // creator_fee_pool
        8 + // treasury_fee_pool
        8 + // total_fees_accrued
        8 + // total_treasury_fees_accrued
        8 + // ema_lot_price
        8 + // total_burned_supply
        8 + // total_treasury_spent
        8 + // early_bird_pool
        8 + // total_buyers
        8 + // total_early_bird_fees_accrued
        8 + // early_bird_valid_count
        8; // early_bird_share_per_seat

    pub fn calculate_buyback_amount(&self) -> u64 {
        // Calculate market cap using virtual SOL reserves
        let market_cap = self.virtual_sol_reserves;

        // Determine buyback percentage based on market cap (1 SOL = 1_000_000_000 lamports)
        let buyback_percentage = if market_cap < 100_000_000_000 {
            // Small cap (<100 SOL): Use up to 50% of available fees
            50
        } else if market_cap < 1_000_000_000_000 {
            // Mid cap (100-1000 SOL): Use up to 30% of available fees
            30
        } else {
            // Large cap (>1000 SOL): Use up to 10% of available fees
            10
        };

        // Calculate maximum SOL to use for buyback
        let buyback_budget = (self.treasury_fee_pool * buyback_percentage) / 100;

        // Calculate how many tokens we can buy with this budget
        let token_amount = (((buyback_budget as u128) * (self.virtual_token_reserves as u128)) /
            (self.virtual_sol_reserves as u128)) as u64;

        token_amount
    }
}

#[event]
pub struct CreateEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub user: Pubkey,
}

#[event]
pub struct TradeEvent {
    mint: Pubkey,
    sol_amount: u64,
    token_amount: u64,
    is_buy: bool,
    user: Pubkey,
    timestamp: i64,
    virtual_sol_reserves: u64,
    virtual_token_reserves: u64,
    circulating_supply: u64, // Track circulating supply separate from total supply
    real_token_reserves: u64,
    real_sol_reserves: u64, // Added from BuybackEvent
    creator_fee_pool: u64, // Current unclaimed creator fees
    treasury_fee_pool: u64, // Current unclaimed treasury fees
    total_fees_accrued: u64, // Total historical creator fees
    total_treasury_fees_accrued: u64, // Total historical treasury fees
    // Fee distribution for this specific trade
    creator_fee_amount: u64, // Creator fee earned from THIS trade (in lamports)
    fee_recipient: Pubkey, // Current creator wallet receiving fees (CTO-aware)
    // Buyback specific fields
    is_buyback: bool, // Indicates if a buyback occurred during this trade
    burn_amount: u64, // Amount of tokens burned in buyback (if any)
    price_lamports_per_token: u64, // Price per token in lamports during buyback
    total_burned_supply: u64, // Total supply burned so far
    total_treasury_spent: u64, // Total treasury spent on buybacks
    // Early Bird Rewards
    early_bird_pool: u64, // Current Early Bird rewards pool
    total_early_bird_fees_accrued: u64, // Total historical Early Bird fees
    // User position (for early bird tracking)
    user_position: u64, // User's entry position (0 = not set, u64::MAX = disqualified)
    user_balance: u64, // User's current token balance after this trade
    early_bird_cutoff: u64, // Max early bird position (e.g., 20) - backend MUST use this!
    total_buyers: u64, // Total unique buyers so far - helps backend detect missing events
    early_bird_valid_count: u64, // Number of non-revoked early bird seats - for consistency checks
    is_early_bird: bool,
}

#[event]
pub struct CtoEvent {
    pub mint: Pubkey,
    pub old_creator: Pubkey,
    pub old_streamer_id: Option<String>,
    pub new_creator: Pubkey,
    pub new_streamer_id: Option<String>,
    pub platform_authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct CompleteEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub timestamp: i64,
    pub early_bird_pool: u64, // Early Bird pool balance at bonding completion
}

// BuybackEvent has been merged into TradeEvent

#[event]
pub struct MessageSentEvent {
    pub mint: Pubkey,
    pub sender: Pubkey,
    pub amount: u64,
    pub message: String,
    pub timestamp: i64,
}

#[event]
pub struct DonationClaimedEvent {
    pub mint: Pubkey,
    pub creator: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
    pub remaining_unclaimed: u64,
}

#[event]
pub struct EarlyBirdClaimed {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub position: u64,
    pub timestamp: i64,
}

#[event]
pub struct SetParamsEvent {
    pub fee_recipient: Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
    pub creator_fee_share: u64,
    pub platform_fee_share: u64,
    pub treasury_fee_share: u64,
    pub buybacks_enabled: bool,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = Global::SIZE, seeds = [b"global"], bump)]
    pub global: Account<'info, Global>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[event_cpi]
#[derive(Accounts)]
pub struct SetParams<'info> {
    #[account(mut, seeds = [b"global"], bump)]
    pub global: Account<'info, Global>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[event_cpi]
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, mint::decimals = 6, mint::authority = mint_authority)]
    pub mint: Box<Account<'info, Mint>>,
    #[account(seeds = [b"mint-authority"], bump)]
    /// CHECK: The mint authority is the program derived address.
    pub mint_authority: UncheckedAccount<'info>,
    #[account(
        init,
        payer = user,
        space = BondingCurve::SIZE,
        seeds = [b"bonding-curve", mint.key().as_ref()],
        bump
    )]
    pub bonding_curve: Box<Account<'info, BondingCurve>>,
    #[account(
        init,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = bonding_curve
    )]
    pub associated_bonding_curve: Box<Account<'info, TokenAccount>>,
    #[account(seeds = [b"global"], bump)]
    pub global: Box<Account<'info, Global>>,
    #[account(address = metadata::ID)]
    /// CHECK: We already check the address matches the mpl_token_metadata program id.
    pub mpl_token_metadata: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"metadata", metadata::ID.as_ref(),  mint.key().as_ref()],
        bump,
        seeds::program = metadata::ID
    )]
    /// CHECK: No need to check this
    pub metadata: UncheckedAccount<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[event_cpi]
#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(seeds = [b"global"], bump)]
    pub global: Account<'info, Global>,
    #[account(mut)]
    /// CHECK: destination address
    pub fee_recipient: UncheckedAccount<'info>,
    #[account(mut)] // <-- make writable for burn
    pub mint: Account<'info, Mint>,
    #[account(mut, seeds = [b"bonding-curve", mint.key().as_ref()], bump)]
    pub bonding_curve: Account<'info, BondingCurve>,
    #[account(
        mut,
        seeds = [bonding_curve.key().as_ref(), token::ID.as_ref(), mint.key().as_ref()],
        bump,
        seeds::program = associated_token::ID
    )]
    pub associated_bonding_curve: Account<'info, TokenAccount>,
    #[account(mut)]
    pub associated_user: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"holder-stats", mint.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub holder_stats: Account<'info, HolderStats>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[event_cpi]
#[derive(Accounts)]
pub struct Sell<'info> {
    #[account(seeds = [b"global"], bump)]
    pub global: Account<'info, Global>,
    #[account(mut)]
    /// CHECK: destination address
    pub fee_recipient: UncheckedAccount<'info>,
    #[account(mut)] // <-- make writable for burn
    pub mint: Account<'info, Mint>,
    #[account(mut, seeds = [b"bonding-curve", mint.key().as_ref()], bump)]
    pub bonding_curve: Account<'info, BondingCurve>,
    #[account(
        mut,
        seeds = [bonding_curve.key().as_ref(), token::ID.as_ref(), mint.key().as_ref()],
        bump,
        seeds::program = associated_token::ID
    )]
    pub associated_bonding_curve: Account<'info, TokenAccount>,
    #[account(mut)]
    pub associated_user: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"holder-stats", mint.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub holder_stats: Account<'info, HolderStats>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

#[event_cpi]
#[derive(Accounts)]
pub struct InitHolderStats<'info> {
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = user,
        space = HolderStats::SIZE,
        seeds = [b"holder-stats", mint.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub holder_stats: Account<'info, HolderStats>,
    pub associated_user: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateHolderStats<'info> {
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub associated_user: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"holder-stats", mint.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub holder_stats: Account<'info, HolderStats>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimHolderFees<'info> {
    pub mint: Account<'info, Mint>,
    #[account(mut, seeds = [b"bonding-curve", mint.key().as_ref()], bump)]
    pub bonding_curve: Account<'info, BondingCurve>,
    #[account(
        mut,
        seeds = [b"holder-stats", mint.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub holder_stats: Account<'info, HolderStats>,
    #[account(mut)]
    pub associated_user: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[event_cpi]
#[derive(Accounts)]
pub struct ClaimEarlyBirdRewards<'info> {
    #[account(seeds = [b"global"], bump)]
    pub global: Account<'info, Global>,
    pub mint: Account<'info, Mint>,
    #[account(mut, seeds = [b"bonding-curve", mint.key().as_ref()], bump)]
    pub bonding_curve: Account<'info, BondingCurve>,
    #[account(
        mut,
        seeds = [b"holder-stats", mint.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub holder_stats: Account<'info, HolderStats>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(seeds = [b"global"], bump)]
    pub global: Account<'info, Global>,
    pub mint: Account<'info, Mint>,
    #[account(mut, seeds = [b"bonding-curve", mint.key().as_ref()], bump)]
    pub bonding_curve: Account<'info, BondingCurve>,
    #[account(
        mut,
        seeds = [bonding_curve.key().as_ref(), token::ID.as_ref(), mint.key().as_ref()],
        bump,
        seeds::program = associated_token::ID
    )]
    pub associated_bonding_curve: Account<'info, TokenAccount>,
    #[account(mut)]
    pub associated_user: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SendMessage<'info> {
    pub mint: Account<'info, Mint>,
    #[account(mut, seeds = [b"bonding-curve", mint.key().as_ref()], bump)]
    pub bonding_curve: Account<'info, BondingCurve>,
    #[account(
        init_if_needed,
        payer = user,
        space = MessageList::SIZE,
        seeds = [b"message-list", mint.key().as_ref()],
        bump
    )]
    pub message_list: Account<'info, MessageList>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[event_cpi]
#[derive(Accounts)]
pub struct ClaimDonations<'info> {
    pub mint: Account<'info, Mint>,
    #[account(mut, seeds = [b"bonding-curve", mint.key().as_ref()], bump)]
    pub bonding_curve: Account<'info, BondingCurve>,
    #[account(
        mut,
        seeds = [b"message-list", mint.key().as_ref()],
        bump
    )]
    pub message_list: Account<'info, MessageList>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: We validate in custom logic
    pub streamer_identity: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimDonations<'info> {
    pub fn validate(&self) -> Result<()> {
        // First check if it's the withdraw authority (platform failsafe)
        if self.user.key() == config_feature::withdraw_authority::ID {
            msg!("Authorized: Withdraw authority");
            return Ok(());
        }

        // Check if token was created with a streamer ID
        if let Some(token_streamer_id) = self.bonding_curve.creator_streamer_id.as_ref() {
            msg!("Token has streamer ID: {}", token_streamer_id);

            // When streamer_id exists, ONLY a verified streamer can claim
            let streamer_account_info = self.streamer_identity.as_ref().ok_or_else(|| {
                msg!("Unauthorized: No streamer identity provided for token with streamer ID");
                HorseFunError::UnauthorizedDonationClaim
            })?;

            msg!(
                "Attempting to deserialize streamer identity from account: {}",
                streamer_account_info.key()
            );

            // Try to deserialize the streamer identity
            let streamer_identity = StreamerIdentity::try_deserialize(
                &mut &streamer_account_info.try_borrow_data()?[..]
            ).map_err(|_| {
                msg!("Failed to deserialize streamer identity account");
                HorseFunError::UnauthorizedDonationClaim
            })?;

            // Verify the identity matches exactly
            if
                streamer_identity.wallet == self.user.key() &&
                streamer_identity.verified &&
                streamer_identity.streamer_id == *token_streamer_id
            {
                msg!("Authorized: Verified streamer identity");
                return Ok(());
            }

            msg!("Unauthorized: Invalid or unverified streamer identity");
            return Err(HorseFunError::UnauthorizedDonationClaim.into());
        } else {
            msg!("Token has no streamer ID, checking creator wallet");
            // No streamer ID, ONLY creator wallet can claim
            require!(
                self.user.key() == self.bonding_curve.creator_wallet,
                HorseFunError::UnauthorizedDonationClaim
            );
            msg!("Authorized: Original creator wallet");
            return Ok(());
        }
    }
}

//Prediction Markets
// --- ADD events ---
