# market-data-aggregator

## System Requirements
Install the rust toolchain and protoc

## Usage

### Run the server
```
$ cargo run --bin server -- -q btc -b eth
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/server -q btc -b eth`
```

### Run the client in another terminal
```
$ cargo run --bin client
    Finished dev [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/client`
Spread: -0.00002
        	Asks    	        	        	Bids    	        
----------------------------------------------------------------------------------------
Exchange	Price   	Amount  	Exchange	Price   	Amount  
--------	--------	--------	--------	--------	--------
Binance 	0.05434 	13.6222 	Bitstamp	0.05436 	0.004795
Binance 	0.054353	2.9     	Binance 	0.054339	34.5398 
Binance 	0.054356	0.3791  	Binance 	0.054338	2.3396  
Binance 	0.054358	0.039   	Binance 	0.054337	11.9173 
Binance 	0.05436 	0.0049  	Binance 	0.05433 	13.6704 
Binance 	0.054362	0.6875  	Binance 	0.054328	0.1465  
Binance 	0.05437 	1.1036  	Binance 	0.054324	5.9858  
Binance 	0.054371	1.7084  	Binance 	0.054323	6.0949  
Binance 	0.054374	0.025   	Bitstamp	0.05432218	0.02562994
Binance 	0.054377	0.0032  	Binance 	0.054322	27.94   

```

### Quitting

For both the server and the client, press Ctrl+C to quit.

### Help

```
$ cargo run --bin server -- -h
    Finished dev [unoptimized + debuginfo] target(s) in 0.56s
     Running `target/debug/server -h`
market-data-aggregator 0.1.0
A basic example

USAGE:
    server --base <base> --quote <quote>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --base <base>       [possible values: aave, ada, algo, alpha, amp, ant, audio, avax, axs, band, bat, bch, btc,
                           cel, chz, comp, crv, ctsi, cvx, dai, dydx, enj, ens, eth, eth2, eur, eurt, fet, ftm, ftt,
                           gala, gbp, gods, grt, gusd, hbar, imx, inj, knc, link, lrc, ltc, mana, matic, mkr, nexo, omg,
                           1inch, pax, perp, rad, rgt, rly, rndr, sand, sgb, shib, skl, slp, snx, storj, sushi, sxp,
                           uma, uni, usd, usdc, usdt, ust, vega, wbtc, xlm, xrp, yfi, zrx, aca, ach, acm, adx, ae,
                           aergo, agi, agix, agld, aion, akro, alcx, alice, alpaca, alpine, amb, anc, ankr, any, ape,
                           api3, appc, ar, ardr, ark, arn, arpa, asr, ast, astr, ata, atm, atom, auction, aud, auto,
                           ava, b, badger, bake, bal, bar, bcc, bcd, bcha, bchabc, bchsv, bcn, bcpt, bdot, beam, bear,
                           bel, beta, beth, bico, bidr, bifi, bkrw, blz, bnb, bnt, bnx, bond, bot, bqx, brd, brl, bsw,
                           btcb, btcst, btg, bts, btt, bttc, bull, burger, busd, bvnd, bzrx, c98, cake, cdt, celo, celr,
                           cfx, chat, chess, chr, city, ckb, cloak, clv, cmt, cnd, cocos, cos, coti, cover, cream, ctk,
                           ctxc, cvc, cvp, dar, dash, data, dcr, dego, dent, dexe, df, dgb, dgd, dia, dlt, dnt, dock,
                           dodo, doge, dot, down, drep, dusk, easy, edo, egld, elf, eng, eos, eps, epx, erd, ern, etc,
                           evx, ez, farm, fida, fil, fio, firo, fis, flm, flow, flux, for, forth, front, fuel, fun, fxs,
                           gal, gas, ghst, glm, glmr, gmt, gno, gnt, go, grs, gtc, gto, gvt, gxs, gyen, hard, hc, hegic,
                           high, hive, hnt, hot, hsr, icn, icp, icx, idex, idrt, ilv, ins, iost, iota, iotx, iq, iris,
                           jasmy, joe, jst, juv, kava, kda, keep, key, klay, kmd, kp3r, ksm, lazio, ldo, lend, lina,
                           lit, loka, loom, lpt, lsk, lto, lun, luna, lunc, mask, mbl, mbox, mc, mco, mda, mdt, mdx,
                           mft, mina, mir, mith, mln, mob, mod, movr, mth, mtl, multi, nano, nas, nav, nbs, nbt, ncash,
                           near, nebl, neo, ngn, nkn, nmr, npxs, nu, nuls, nxs, oax, ocean, og, ogn, om, one, ong, ont,
                           ooki, op, orn, ost, oxt, paxg, people, perl, pha, phb, phx, pivx, pla, pnt, poa, poe, pols,
                           poly, pond, porto, powr, ppt, prom, pros, psg, pundix, pyr, qi, qkc, qlc, qnt, qsp, qtum,
                           quick, ramp, rare, ray, rcn, rdn, reef, rei, ren, rep, req, rif, rlc, rose, rpx, rsr, rub,
                           rune, rvn, salt, santos, sc, scrt, sfp, sky, sngls, snm, snt, sol, sparta, spell, srm, ssv,
                           steem, stmx, storm, stpt, strat, strax, stx, sub, sun, super, susd, swrv, sys, t, tct, tfuel,
                           theta, tko, tlm, tnb, tnt, tomo, torn, trb, tribe, trig, troy, tru, trx, try, tusd, tvk, twt,
                           uah, uft, unfi, up, usdp, usds, usdsb, ustc, utk, vai, ven, vet, vgx, via, vib, vibe, vidt,
                           vite, voxel, vtho, wabi, wan, waves, waxp, win, wing, wings, wnxm, woo, wpr, wrx, wtc, xec,
                           xem, xmr, xno, xtz, xvg, xvs, xzc, yfii, ygg, yoyo, zar, zec, zen, zil]
    -q, --quote <quote>     [possible values: aave, ada, algo, alpha, amp, ant, audio, avax, axs, band, bat, bch, btc,
                           cel, chz, comp, crv, ctsi, cvx, dai, dydx, enj, ens, eth, eth2, eur, eurt, fet, ftm, ftt,
                           gala, gbp, gods, grt, gusd, hbar, imx, inj, knc, link, lrc, ltc, mana, matic, mkr, nexo, omg,
                           1inch, pax, perp, rad, rgt, rly, rndr, sand, sgb, shib, skl, slp, snx, storj, sushi, sxp,
                           uma, uni, usd, usdc, usdt, ust, vega, wbtc, xlm, xrp, yfi, zrx, aca, ach, acm, adx, ae,
                           aergo, agi, agix, agld, aion, akro, alcx, alice, alpaca, alpine, amb, anc, ankr, any, ape,
                           api3, appc, ar, ardr, ark, arn, arpa, asr, ast, astr, ata, atm, atom, auction, aud, auto,
                           ava, b, badger, bake, bal, bar, bcc, bcd, bcha, bchabc, bchsv, bcn, bcpt, bdot, beam, bear,
                           bel, beta, beth, bico, bidr, bifi, bkrw, blz, bnb, bnt, bnx, bond, bot, bqx, brd, brl, bsw,
                           btcb, btcst, btg, bts, btt, bttc, bull, burger, busd, bvnd, bzrx, c98, cake, cdt, celo, celr,
                           cfx, chat, chess, chr, city, ckb, cloak, clv, cmt, cnd, cocos, cos, coti, cover, cream, ctk,
                           ctxc, cvc, cvp, dar, dash, data, dcr, dego, dent, dexe, df, dgb, dgd, dia, dlt, dnt, dock,
                           dodo, doge, dot, down, drep, dusk, easy, edo, egld, elf, eng, eos, eps, epx, erd, ern, etc,
                           evx, ez, farm, fida, fil, fio, firo, fis, flm, flow, flux, for, forth, front, fuel, fun, fxs,
                           gal, gas, ghst, glm, glmr, gmt, gno, gnt, go, grs, gtc, gto, gvt, gxs, gyen, hard, hc, hegic,
                           high, hive, hnt, hot, hsr, icn, icp, icx, idex, idrt, ilv, ins, iost, iota, iotx, iq, iris,
                           jasmy, joe, jst, juv, kava, kda, keep, key, klay, kmd, kp3r, ksm, lazio, ldo, lend, lina,
                           lit, loka, loom, lpt, lsk, lto, lun, luna, lunc, mask, mbl, mbox, mc, mco, mda, mdt, mdx,
                           mft, mina, mir, mith, mln, mob, mod, movr, mth, mtl, multi, nano, nas, nav, nbs, nbt, ncash,
                           near, nebl, neo, ngn, nkn, nmr, npxs, nu, nuls, nxs, oax, ocean, og, ogn, om, one, ong, ont,
                           ooki, op, orn, ost, oxt, paxg, people, perl, pha, phb, phx, pivx, pla, pnt, poa, poe, pols,
                           poly, pond, porto, powr, ppt, prom, pros, psg, pundix, pyr, qi, qkc, qlc, qnt, qsp, qtum,
                           quick, ramp, rare, ray, rcn, rdn, reef, rei, ren, rep, req, rif, rlc, rose, rpx, rsr, rub,
                           rune, rvn, salt, santos, sc, scrt, sfp, sky, sngls, snm, snt, sol, sparta, spell, srm, ssv,
                           steem, stmx, storm, stpt, strat, strax, stx, sub, sun, super, susd, swrv, sys, t, tct, tfuel,
                           theta, tko, tlm, tnb, tnt, tomo, torn, trb, tribe, trig, troy, tru, trx, try, tusd, tvk, twt,
                           uah, uft, unfi, up, usdp, usds, usdsb, ustc, utk, vai, ven, vet, vgx, via, vib, vibe, vidt,
                           vite, voxel, vtho, wabi, wan, waves, waxp, win, wing, wings, wnxm, woo, wpr, wrx, wtc, xec,
                           xem, xmr, xno, xtz, xvg, xvs, xzc, yfii, ygg, yoyo, zar, zec, zen, zil]
```

## Remaining work
1. Logging
2. Error reporting
   1. Maybe use `anyhow` and `thiserror` instead of adhoc `dyn std::error::Error`
   2. I ignore errors when reading from the web socket streams. I should report them instead
3. Handle ping/heartbeat, reconnect request etc. in the exchange connectors
4. Support more than two exchanges
5. Share code between the exchange connector modules
6. Graceful shutdown
7. Validate currency _pairs_ instead of individual currencies in the command line arguments
8. Make it so that it doesn't open a new connection for every client

## Afterthought about design
One alternative design would be to split the server into different
services for each of the exchange connectors and one service for the
aggregator server. Connection maintenance and configuration, for
example, would be much simpler in case of the exchange connectors in
that design.
