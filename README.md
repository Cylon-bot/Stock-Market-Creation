# Stock-Market-Creation

The goal of this project is to create a generator of stock market database to help backtest your bot (or even manual ?) strategy in any market situation you can possibibly imagine with infinite years of data.

## Okay but why ? 

Market is hard to predict, nearly impossible. Is it random ? No, but it tends towards randomness. If you test a strategy across 2 year worth of data on EURUSD and the result is about 10% per year, great, but it probably means nothing unfortunatly. Why ? 

- Number of trades: if you have taken 100 trades per year, your sample is hardly enought.
<br /> 
- Market Bias: If you test your strategy on USDJPY between 2020 and 2024, your market bias is bullish as hell. but can your strategy perform the same way between 2014 and 2020 ? maybe not.
<br /> 
- non efficiency of market: A theory exist about the efficiency of the market, but investors, including the likes of Warren Buffett, George Soros, and researchers have disputed the efficient-market hypothesis both empirically and theoretically. Behavioral economists attribute the imperfections in financial markets to a combination of cognitive biases such as overconfidence, overreaction, representative bias, information bias, and various other predictable human errors in reasoning and information processing. Empirical evidence has been mixed, but has generally not supported strong forms of the efficient-market hypothesis. Conclusion: Thinking that an algorythm can and will perfom for ever on a market with 100% chance is probably impossible, especially if your backest takes only 2 years worth of data across one market.

## So what's the plan ?

Imagine if you could test your strategy not on 2 , 5, or even 20 years of data but an infinite numbers of years ? Imagine if you could generate an infinite market bias ? or even generate infinite number of market by playing with factor that simulate (the best they can): overconfidence, overreaction, information (and so on) biases.

That's the idea of this project made in Rust! 

## Efficacity of this generate market database

If I perform well of this generate market database, do that assure me reward on the real market ? YES OF COURSE, BUY MY FORMATION, ITS 100% SURE (wink wink, telegram scamer).... no of course not and for 2 reasons:
- This projet is an experiment, and experiment can fail !
<br /> 
- The generated databases, are construct thanks to a configuration files that YOU create. If your configuration is very oriented, it will be really easy to makes something work with it: for instance, you specify in your configuration files there is 99% chance that the market is bullish when a new candle is print, well spoiler alert: that's not possible in the real market and a strategy based on such market will be pointless.

And always remember: **One day's profits do not guarantee tomorrow's profits !!!**