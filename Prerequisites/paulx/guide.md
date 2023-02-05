  Programming on Solana - An Introduction | paulx     

[paulx](/)

*   [Home](/)
*   [Blog](/blog/)
*   [Tags](/tag/)

[paulx](/)

* * *

*   [Home](/)
*   [Blog](/blog/)
*   [Tags](/tag/)

Programming on Solana - An Introduction
=======================================

Thu Jan 14 2021

*   [solana](/tag/solana)
*   [blockchain](/tag/blockchain)
*   [coding](/tag/coding)

Last updated: 1/11/2022, 9:03:58 PM

Time to read: 65.41 minutes

[#](#intro-motivation) Intro & Motivation
-----------------------------------------

This guide is meant to serve as an intro to coding on the [Solana (opens new window)](https://www.solana.com) Blockchain, using an escrow program as an example. We'll go through the code together, building the escrow program step by step. I've also created a UI you will be able to use to try out your program. Additionally, you'll get to play with the (shameless plug) [spl-token-ui (opens new window)](https://www.spl-token-ui.com).

Most of the info in this blog post can be found somewhere in the docs or in example programs. Having said this, I have not found a guide that both walks through most of the coding theory step by step and applies it in practice. I hope this post achieves this, interweaving the theory and practice of solana programs. It requires no previous knowledge of Solana. While this is not a Rust tutorial, I will link to the [Rust docs (opens new window)](https://doc.rust-lang.org/book) whenever I introduce a new concept. I will also link to the relevant Solana docs although you don't have to read them to follow along.

Important theory will be sprinkled into the post like this:

> On Solana, smart contracts are called _programs_

and at the end of each section summarized like this:

theory recap 📚

*   On Solana, smart contracts are called _programs_

I do not claim to explain _all_ topics but hope this will be a solid starting point from which the reader can explore Solana further. If you're new to Solana and Rust and want to finish this post without breaks and leave with a solid understanding of all concepts discussed and links mentioned I recommend allocating an entire day to the post.

If something is not working and you just cannot figure out why, have a look at the final code [here (opens new window)](https://github.com/paul-schaaf/solana-escrow).

If you find mistakes or would like to give feedback, please do contact me on discord [paulx#9059 (opens new window)](https://discord.com/invite/pquxPsq) or [twitter (opens new window)](https://twitter.com/paulxpaulxpaulx).

[#](#the-final-product) The Final Product
-----------------------------------------

Before we start coding, let's look at the final product to understand what we are building: an escrow program.

### [#](#what-is-an-escrow) What is an escrow?

An escrow smart contract is a good example to look at and build because it highlights well what a blockchain makes possible while still being easy to understand, allowing us to focus on the code itself. For those unfamiliar with the concept, here is a brief explainer.

![](data:image/gif;base64,R0lGODlhWwHxAPcMABYWFhwcHCIiIiYmJjIyMjU1NT09PYmJiaGhoeXl5ezs7P///wAAAAgICA0NDRISEioqKi4uLkJCQlpaWmZmZmxsbHd3d3h4eKSkpLa2tsXFxfb29ktLS319faurq7Ozs7u7u+Li4unp6QcHBzs7O0VFRU1NTVFRUVZWVlxcXGJiYmlpaXNzc4ODg4SEhI6OjpCQkJaWlpmZmZ6enq6urr+/v8PDw8jIyM7OztPT09bW1tjY2Nzc3PLy8vv7+zg4OEdHR09PT1VVVV1dXWpqapSUlL29vcHBwcnJyc3Nzfj4+AUFBUlJSVNTU2BgYHJycn9/f4GBgYaGho+Pj5KSkpubm5ycnKioqKysrLGxsbm5udDQ0NTU1Nra2t/f3+Pj4+jo6PPz8wEBAQQEBAoKCg8PDxAQEBsbGysrKy0tLUBAQFBQUFhYWGNjY2VlZW5ubnBwcHV1dXl5eYCAgIWFhYyMjJOTk5iYmJ2dnaenp6+vr7S0tMfHx9LS0tfX19vb297e3vHx8fT09AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACH/C05FVFNDQVBFMi4wAwEAAAAh+QQEyAD/ACwAAAAAQQHxAAAG/8CFcEgsGo/IpHLJbDqf0Kh0Sq1ar9isdsvter/gsHhMLpvP6LR6zW673/C4fE6v2+/4vB4d8u3/gIGCTDEQDAwmOgs8HlUcG4ORkpNaNgw1PjgcBQsfAlMIJQwKlKWmp0oIo0I6LTwCiIsGACmkBy0ntD1EISiiu6jBwpQ7DAMtmEIuADgiDBU2JgYLKwwwIMdGOavD3d5/rQGHCJ2fLwGsoyslQh7TRdvA3/P0dTooo57Uh/w5KyzptHGrR7AgmhQqhvC4pK8Dp102fFBIuEAGAIGkDGrcCMZishb5LoK4tAGkRAd9DNSCN5Cjy5dWKvAj8GERgAH7jIGgNuAQgf8EGGEKHSqlGRIROSBRu7ChD9GnUMX8i0q1KpcMyaxq3cq1q9evYMOKHUu2rNmzaNOqXcu2rdu3cOPKnUu3rt27ePPq3cu3r9+/gAMLHky4sOHDiBMrXsy4sePHkCNLnky5suXLmDNr3sy5s+fPoEOLHk0aMr/TqFOrXs26tevXsGPLPl06S0u0t2tLyW2Wt+4nvskG/81kuFjjxJMgB7s8uZHmXqE7HyKda/Xp17VmT769avff36OGrz3+aXnS54emF70eZnvQ713G9zx/Y33O9w3m17yfYH/M/9EToGUDflMgZQd2k6BkCwrToGltPfiYhKdQ2JiFpWC4mIaTcBj/2AvyUJdKao9AMYNSScRDxIkLkEDOENgkEcMJVtCww27TKTHCCBaEOFwCCTgEZEbAAaWEigG1iAE8MtJYRQSNROEhYDA0sGOPQiB3zhArwCABAipkIEQKOx0wQAAttGiMCBIJQFM7BBRgwUCzRCACCevocgONM3SgAgU8lYDQirjoMuZNMghRwAwQyIQmCX74QEJEOuao42lMaQnBEBLE8qaSNzCDgQMajERDmx6oYsNCFagykKmSWhMDQxctU0INHjjwwgGxcGlNDQLAwJMNqvQxTqiukFSDcVN+1els0J6WSxNbKkrRpzTliklSCqxizZit2llRS6vgqagMMS4D/+6YTq5bTic34kDSt0ouRU27yhEIxgOY9qApp4nWW28Kh1SwyygJoOaLwe8qVO6LX6Y7QZZRHoDvRAEBa8y8itQ7qg8BRFlpZe+dcyUw/56r8gKX9KGAtt0CRVJTO+DSDrlAuXiuxCtjzCWg76JJCqk+cAMlxcg1G90XVjJFRMoC+/LyJV6SYoCw5aKggA5CE/3L04roDGqtE1f00zYXo5SAuQ40MgPHPfe6hNJdlRxiltRuuvJIiAzgQQLiOACpmkAG7mQuiLREwihiR0w2K4YgcnFPa44rOAcocXOBsjPPra8Z4bHJi5HaOMWK6FfcaMRUpp++hD5N0G3dGbLH5P90FGYKG/vnZdReBVagiOmE71b9R3wex1NlfIS8k5H8Hc9DtTxb0ZtHO/MkX09982NUT4f36mm/FvhCTT8+92KQH4f6L5mvFvvyif8++mHA74b99smfFv4aub8//fvCHoL0hxsAMk2Ak/FfAbMHOgQyiIBn4Z9+INgbA3ohWhjMoAY3qEELWoouEvygHkIoQjyQsIR2OCEKv7fCvKiwheuD4V1eKMP71fCGOMyhZ2iAr9QJ71BKycYQWKTDYfBwCzPQ2tM6wBMUsYxbRaRDBkgQLEgwqooKwNNNbkSDCERAagrB3K06QUUQnYRMsqCFDoqhKyIsyyNEmJM5zsTEKL7/gVRr+0AISAUsBCSsA2sDyK8ggK8SuKAkNMIjBz4Qig24Yyk8sAA6ZuTEfaSpCMwoGgZ2wIED2NENm4CBIrwkAxlwICGrSCLsXtBDGliME6FURKigAYkGtCAGs2KEEomQC6CBDVhJ+iQbMKCCS1jARbgUUypPkEQ48dKQZmpHMcWEgxYUgBOywuXLdnkzS/wQb81smDDV8AlI6i0FMUiYM7MGtiaWswIWuKVNsngiDZjDBLwgyTlQ90Rn8EAidRxnGuQorgqIA1LOcKajJDA5AVCgbQTdwQ4CQMc0fmsbAS3ASiTFzQmghAU++adA1TCk0aWISCZVSkldp5C7FUWk/yONqUxnStOa2pQSRHQCCKCIBAgYZRE1OSKMSmQEoUohpzfdQiafgM4l+HSIWntkSotgVCnxNKlYkOMrrJmBFShDTA+dwA1yxYy1qfFQHv3pTVzQxV460mAosKYIuvjFdvHglIa6h+CSEleHGqMDdQQsVq1QVkTgQKrMxIAEHAmpuAK0ZpOUQA6W8VNK0uBbAmAkO+K0k0Ha9Vt/Ghg+OauAkFnCD0QbLBUKCxTFjskDs6yJvXSFS8bJtnON7OJrj/imVX7WDzkopysNAMsoZdK05VTtFFhLRpVVM06zZas2l5ra5gpVBdfYbETCaUqHBURQfSou3pZCgUMqd7nNWP9FjBaCARd4Up3YrZcNPgHPTuAWn9fFpXZtciNBOewG9/LXjQwqsExyrbrnhUJajWYMCPyNolX0iD3FsaTgNsBNUMRofnlLqYV+9kwDUISjHuqODqOkRYtNsBZIx9KptngKrQtmS50qMhUPg2tR6AVObPwNSpkoUjweYZCVN2TxFFl6R7ZeksO35PI12clPbl+UpTxljtCwyk/Dcv60PEEuF+TKXAazlsWMZTJX2cxTRnOU1fxkNjfZzUuGc5LlfGQ6F9nOQ8ZzkPXMYz7b2M8qBnSCBY0gDhr60Ije4DwSzehGO3o1DnRQhAgNBRJSGjiXHt6iJx3pYGQ6dp8uzqb/qRdqzxmI09s7Nak7jYpSX4rVFUL1+VQ9Plfni9bvszUSLC3r+eF6f7o+Aq9XnWpvBPs5x8bkqGsNa1Mk+2nPFtGvcRPt8Rq71/+bdgSrzbJl57rZGcL2Aq9N7FmTm9nFVpC41xCBE7/LZnsY9h1ncsmiDu57EtrGzE7Lg44JWdvkrHAuj+DaOhwCDhZYWED6lEaxLgIfY7yjt9uQ2oTtRK8F0EA5CDYpORz8joxUrwPMtgE1miAh16zB5lC6Bnnfr0te2nEAFrnyUYmVikC+34PWe9GR+ykDq4ikvEQKgFPZEOBpuJwoAqADoKMMA7mCiIU+3gaOw4Idp/VTIVaE85oYSBzpaKjuKbd+roL3Ew5UZ8OvVB6SAGtuJEb6KcUnrnbZ3uu0WSrVjotRSbU3SBVA1lXWa9HGdXQLRED3t9rp3nJRhgNEbXT6qE5VAXasr0H+5dKkfJ4Qi1RuVAVDO+PJSW8/zMoB3yKrTjxOPBbTLIZgj8NVF5HzeYObEtxOe7bP/e10DyP3uR99WXQ/bnWX29e8B/btO7TuCsb+OMF/fliIH0HhCyf6yaf28iUB/O1Hovu+l/Txd298dJu7/L0//++bP3zrj4X6zs/+tr0/CPCrX/zmRz76lR9+T7NfOO4HfY82gARoaItWgAiYgLIRBAAh+QQFyAAcACwQAAAARQHxAAAH/4ALgoOEhYaHiImKi4yNjo+QkZKTlJWWl5iZmpucnZ6foKGio6SlpqeoqaqrrK2ur7CxsrO0tba3uLm6rF9Ku7/AwcKQRRAMDEFdC19Xk1dCQlgbw9TV1p0aDEZKW0wFC1kCklkMTxgCF9fq6+yNVgwKglxRXgLI4AYAQ/EHUUH6YQg9ISKICoB2CBO268JgQJRtBQEkAVPuiLcFRBgUMYKOEJdeGIUoHEly2LwAxxCAEzclgDx4RIAUlHCIgraSOHPq4jIBXjiMx4JuIfLkZaEkAAhw0cm0qasJTgZ90fYTyrdpGpQQjejRwQFfTsOKLVXlplafB7XcjHIwY68f+/8GUTDQpa6ysXjzbqoQlECWZUmBNtw295hSQgOCHtPLuHGliYjALBW0FaTjy5hLbc3MufOnDBA9ix5NurTp06hTq17NurXr17Bjy55Nu7bt27hz697Nu7fv38CDCx9OvLjx48iTK1/OvLnz59CjS59Ovbr169iza3+kuLv37+DDix9Pvrz589+3v4JHm716Vu5lx3+faj5s+/RN4Xe9P/+o/qwB6B8oAqpW4ICdHIiagghqwqBpDzZ4SYSkUSghJRaKluGFkWzYmYccOgJiZiOGuEiJl6FoIiIqNtbiioW8qJeMMApCI143wpijWDua2KNTP3IYJFNDSlhkTkcimGT/SUv61+RIT9IXZUJTqldlO1dqlyVY1myJHYBXeMdEJFZMo8gW85W5wA8qDaJWIkU0YYk0kuD3VY2WCJhAAlbtGQ8kBXIxH3tsHgWnnJVE0EyHhViwRAN45ukOBHJRIQECQ2RAmRYLHDBAAFKs2RAYWgng10wFOEqIBKMaEBNASMhpBRROUNDpAEDUKhCtAFGWlJoFIABBRqD+ABZdJ8rVwKNURFrJgS0NwioFXZy65hVISHQOEmpJc4EAWCCgzVQVCBtft0rkU0URVB0kBTJGhDnFu4j6ulFHFfygAQbspZQtPTcZUSBMywbl7LOTShvVtYL4FeY2W5Dab5tDlEsA/1dSEQpFw1a8yRZlRa1Zb0gzgeMFNzdpxDHJAzGCzKPoxdyXkgmvzDDDThxTwTTwJNAdNDuvFGMCN1/q8QQ2LnpAELsaxVFDKU926jkLBLCoIgQX/ACBNDMSrc3WjksqFgBUocDEWHlBK8ZGFX0vyUWPTMHCZVUdtAPbuKdo0ge6dwHMzXri5WvQUmozNAqUdYWl8RjQLKFIc2F1ttsAMehSjnN8NFdoyu3AW/s00Mw7eUttNlBIu9zoo58Mzl/NN6v1cjMoOWCsqHvWjmgTx/xTSD4JFHrt5oD1Lndio3Zq/Od+B2xWsoZM0XrXlpDqkfWG1HV9Jnc1asEGlskjPv8iGRzUSJGut5b+XulI4mngqutHfUmgOfOXiKesv5r+wqA/f2z+GxD/ghFAJ9WmgPkZIDAQKKUD5u9/93mgAB0ovwm2R4IGvGAFMzgbBr5Hgb/woJUoWAoQQoiEpDBhaVSYCxFuh4W4cKGWUPgfCBIOgwmkoShgqCEdhoKHngFiLWSYHSHSgohfkpkSl8jEJi4RhwdrjhGjyCQqPmeKVlQIFrOIkC1ykR1e/KI6wijGLpUxOWQ84zDSqEYCttE4bHzjLuIoxxbWcTh0vKMt8qjHI/YROHz8YywCKcj1FLI3hDwkfBS5m0Qysj6PzI0jI7lBSmrQkpfEpHw0mUlOevL/k6C0DRZGdoktaEphcrnYINQUSmpkgZRzEombzPK8qkWslavIwA86cite/uBbBVgK2SKAOKkwwXYT0SUvbTIBJCyDd0D4CEpClco1fU9ZLPnUxnBZwsplYSoaOAK4KAKFLwShKBpBysiAECopyAlvwcuAsDaABWMRRW0uiZOZBhEA3w1NCV6p1p24KQpvUGEplipCFZgQFfZYoQk/6RTTCBEuA3wjCAQ46EqIcD+vrAseDzUEOU6JGC5wRB6GI6goMJAzLViATQrllEMhOiahKWxWV2FpwKRAgG9URKGJk+U/hxZSm6qUaxixgAYMN4QiUKRkkMvYUvKlAHEkNQp3/4IHE8q0VCUUtSt/KilFTja3o4bCUQWIgDL4YjuelYytEjgeOvCGVrVKTpv4AIDztilVQ/QEDE8wzMnMCgo/bS8yYT2sLww7PqkEpHqDJaxkJ0vZylqWoKx0hBZumQgIcNak+KCoTA4xSkpk9rJ14mwjnOBURXh2lSIprWJFS4kAqBa1j0BrPfzh0k2VqplhkkgCoCm1fkoEpR4lwD9kRRAhfGyYxXxmRoLWDWQmda7o2CYUpIdbl2UFXs+gzOJoEt4JSO+equLtAZ7HOLJhQHLylElPZYoAdWbsvOlgZxjcuSYIaEpbkAJoVrqbrLMRjaWCCAK2KjKNpgJ0u+z6Qv+wbASZ0JINZKWdWgRK1rYtWHW83whbxCZnVQK7VpocLlQSeHpRp/7UxQP+HGxDm9QMZ6WoC82YUXK1rhAfgcJXjYKJTzQR9mhBHF7QCFYBowDWMqyrFdiZWiocUtmy1sZAzlVfnzCmtfgYyIKS8ZAT8Ve9NcS/1cKr4paqVwwUz1Sf9WhNryzfAcN1oss48wCmmhSb1NPOMk7rmCVBtNk61tCRCF/bEP27qw26GpJLdBNS+mhr/JhMXKo0LCap6UFwutML+HSnRa1pUlfa1I9G9aBVPWZWD9nVJoY1gWXdXVrj1taoxfVldW1ZXlfW15QF9mSFLVliE9bYZuWjE5f/zexmp+dDzo62tKftnRN+yIecwTWy41ehIGKbRN7uZLiz/W3MaLvcKRr3JleobgBam9zivra8O/hucMcb3vhed7fnre/RnPve9s63u9nN74HvW+ARJDjCb6jwgPe7hwVP+MEdbnB/t1viFo84wydubnQ75t/0DkUExAzluOgC5K5wQF+ETFoD7DEUaDKLOJcxmZNffBXwBIPiWv5yUFgAaEaZVfGa+cykVLgVKEc6nSjCKZ5Y9xxDsO4gRZ6F8gVk5mvThwbOKTIj/C2xOL+5KhzA0TgNwBf9PAIUPhemZlo000gHxUk34Lwa+w0pgy2bITU+dmJazrZvSlqYrr6l/BtF3VT3wLp5zz5LxbQ27nyvz/3SZY6UwvR2Ap566w4aL7TYfbFUCDz4wD52sUs+lTOn8BWsypB9pvwzYgZoFRQv+w3cU2XkqPkiI4+KgG7hXY8z25v45a3RbvoTWka9A0hWluQFV2V7X/jYV+4Ldqmcq/YYjCyqVOhDH5/3sbjtMuD++oZ3HODnl7766p3+h5t+Rh530fv3x/50o9/+6g9Q/T8ef8YkHeMQl3/0Z374537ghyP9B38HmBrC9n8bl3ECaCD7J3/3x38LuCAT6H8JmBcO+DoEaIEhx3EFWHEBSHEAOH8IWIEUGIEMSG0u+ILhEUQwOIM0eB6BAAAh+QQFyAAMACwAAAAAWwHxAAAI/wAXCBxIsKDBgwgTKlzIsKHDhxAjSpxIsaLFixgzatzIsaPHjyBDihxJsqTJkyhTqlzJsqXLlzBjypxJs6bNmzhz6tzJM2cIQT2DCh1KlGMRNGLEBPGzAJCHiUWCtNFTtKrVqyyPiDGiJAmHAgs+CJB49ACerVjTql278awCgX/mABKgtKkBAEPe1ukQBG8ggkHqCAQSh63hw4gX/hEzYA5XgXQAJEkg5s2RrwuIiCkCQoAdg3/OgkhMujRiP3ACJEUQdqydM3DFJHADRGAeAwaHJO1jurfvtH6EyBabOanxPkTgxEYIBPfv59BzDmkzENBW4nLAAj2iJLltAIAf3/8JEL28+Zh30HYfDhsE2jmw3ZQBlMBA3oFtltbvcL6//5SaJUXAB00BMEBxu2U2gIAJEJQEAEkJ8dN/FFYY0mQIicCUQG7IIciEoP1l4YgkAlhYiSimmNVjKrbo4oswxijjjDTWaOONOOao44489ujjj0AGKeSQRBZp5JFIJqnkkkw26eSTUEYp5ZRUVmnllVhmqeWWXHbp5ZdghinmmGSWaeaZGhmn5ppstunmm3DGKeecdNapJppDidGjnngGxeeOf/a5U6A5EiooTobemOihNS1ao6OMygTpjJNG+lKlMWJqKUuavtjppil92qKooJpEaoqnljpSqiWyqipIro7/GOurHc1aoa20prlnrpLuyitMuP4X7K8VDdufscRKhKx5yyb7ULPRQessQ9I+V+20CV3rm7bYGsStad92O1C4pJErrrmIoYutuoax6yykHrDJQUR4AKVQH4TWu8APeTgIHkJ3rGGRHn8oK26ois0mQAIMRySbYoQ+bABrDiYUlUVpPOXwwShhWgca+NmhBgJD7MEhVwcMcMYc+zImQncCDGgbAQXEEehdaYhgnxp+ISEwHny5oSBhAg+0V1/3iSDfxIPhgYZmAaSsRNMLuZusxyAPVpfM+3rAh2TxIuEeVdnpgcBWIVR2dqBjK3HXHUVcB15kQBiRRxl71YXfZp19//bGD3yc3eBqSHi2GFdGVGp1kHfZ6fjjdzL02kBqUNd102F3NfjgFE/nNGQRN/gDf10nriDV+xbNYW2thUXfg1xtJlDG+76RmRDUQq677s7JuPhBWFN+B+po9KtbGbYrIFvaagrBRvJIhH556XOzIVAZGtehembKtW5EGoyh9fDl8S4QgMYK/Z4+jeoXFDzxFKPNsB4ADP+wen/EFcR37otO8cidWQAdrDc92hDEOwIETwCShz0ljI92ykuPvdank/YN6ibvK+DP+DavrgVCYhLyQ9RgpzPp8Qtz1YMMAfLnl73Rxz6CaKBoHLghAhRBawSs2gUphcGGfEx432HMAP88kADVlOEHb/nB8oxYtDVEiFBKrE+/qCfApCFFKduTD4MSeEQOlIGGoHtK5qi1Q9/18CIvq1iGQNSUBmGkYLnxEBsV4MY0HoQ4DbEg8Nh3RtJ0CCqNcYgevcVHmwyyLSyCSB1MJsgyZqqPOjokQSRpSEgWypEwOiQl/YRJT1kSR5tcQChnoklf5WSUvaokj0KJypiUcpWddNErARXLUX1SUbVU0SwjmUtU3dJGrCxko0yJKGHSpJWn7CWKdnnJChqTlMT8pS6l+UxVmtGavHQmD7HZzGRuc5iw1OY1wUlLcT6Sm6BUZquo+c1itpOUu4unPOdJzzWpk0TILFU+XbL/z031k1McA1Y1A4qwdxLUS/88KLMUKpE3fEYjIODNQAYgtAIRyGhVM04BNtS/PDE0InB4aEaGcMPqoOUHlltO+nhTnxxOEhAe5VjKmAYzmYFgOsj7C3zUMDKC6EGJuEsgTZ0YhwYVzi/xkswBo0YeghSgLjst6bjg+AbcBQd5TCFDG/CChJ4ktEf40hB5buMHwdHvA304A1rBJrtxjSYwYU2r0lgWBw8ZjgPUEYJgCvLURBaIjgHgw/e6Oq415ActUgkBHOZ1REDAx6sBxcv/2AC3ESKRe3LoHgdE2hQEwMeua6DYxxzTlc3c4Q3xkeq42mrSKkImedezwwcQ8APc/4xPNg0E4z1plYCoyCYNbbBDETjzUw7ZgaSou54b8vC8znrxZTflKt6EWy/kLlVw/uLea18aG61MLYZFhOP4PloTIxRtiAewHBqS4IEOBre4Isht6wRRs5tObYgcQNwK5+Na637nJ0FQQ3bxyLXrsVQOCqTK58hwgPmS1yZKRMO8FFaZsLj3M0o8ompptsD5RHhe8aIZgZxmIN6kh5FF1FeKnYqbDHdwtQLiQ1iOY76nqucjIW3LW7o70RN1liFp657ytKslOFbHjgcxckHmqORAcBQu360IfR7S5DqCJMe6IsjkEigi1y4kMl/86xwfrJOP0TQOAZBZepVIup321P9oKmvzAIbawqPm1WVOlYt8/3CGnOomvwYRwFn/usjWtBDNBSazS+I6VsAJDrV92AN7jiBpkX6NPvNh9FwDkdnS4RW8S8ZiQS6mvdD8q7UI1JNDaRiCjJEVu9NCpWRfJpXKkoxl5kOOZjnbh9OKL7QvG62MHwQ31OY6N+O1De6GShX8jAzBf1E1Hb4H5VqPh2Lv6klvnwtc6iJnr5KxLmUHkjjHFM+5D9tDauIgluEO99gFga+WcafU6RXIeVtljbTxwDrABne4Es32TuzbtfTeNwmrzjV85dtp60T0vB+oW+v4qz14z9sgJNNuWMl9IBV+ULsSA4HBdZaEdfXkw13/6UuFEx5Y+gpIqiJEA3BzhjMQH1HQhlYqG/jr05yFWs8QwnVywyrt4rRQ5bAVOE+qjOQQJXnJSH6y5i4idSk3/WqK3tJXs16ugXK9Vl7/+ka23jGxi4TsJ0E7ntReErafye2rMntI4H52ucMq7Ha3CN3nnneP7P3ufefI3z8y+DAVHuyBHzveE78xgzKeIofnSO8er/fFUz6Plr+8Dh2v+WdlvvMIibziQT8R0WeZ9I0fJ+o9z/nVZ+vzrr8ejQIw5tiHfva1tz0hZ7Qw3ZOR9032/e5lNICqC3+SuD/+64Gv/Nszv/nDj9EKoR99GE2f+sh/PvZlr/3tmz4jgNv+/7hopAbuiF+U5Pcr9b+PkfKfH/0zkvj52X8RQM8//e+n/0QYGYSLThD6+hcRi1E8/WczHbd+MaJFZEBj2BeAArgm/XZ8dcBg6PcaDdYiqMWAzTcGCxg5siQg2IdgauIhL5IaZjAaDeiBnpI1IciBY0CCMCJb85cUXfZ+MIJgMGiD0leDXlVPPviDQAgnrRKERFiEPlh96YRP0QR/5SQrS+iAgrdO4ZR9TWghUDh6VJhNTjiF48eFt/KESygsYOiFFHKFp9eFVfiFZGiGGcGGWdhNaKiFVjiGaViGdCiHaliHbngRe8h9ehiG/tGHxYKEuKSEawiIx3KHcJiHeMiEjf8ohodIhpD4h5IYiIqYhFtIiXU4iY0oiJAnhZr4iJYYiS8BPt/lXXzBE4L4Ox3IGEIXb5d1TISYEviCP180ZbtlLa7EGiIQNzKGcbEITe4DE3HwPATkXUBjF1jVFMIhf/wEivzUbJQxGle1Ua3xZwF3KbPYMbOVbl+UimcwFVKROkZgMzvWEqvoSkQANDxzX/nVAd+oNrUVZc84jIsWZrKDiiUzPnIQPTBlPugDUIZ4KWnQHCUmaSKCPeXTFMmGjttoEjsXM3Whj+PhU2uiWiuRjsAijXh1FMJzG27lSg/ZdrJlN+xRRabTWe6RRhhSj5kYjc7mXQZ2NyolUPbIEmfdE2UK+Y150UDJEV+C8QENqRIaeSl24AeogTcAWVpKcDdkE4EuGYcqQRhLpY9cZI1JxVoOOZD8pFEnEjdlkI90kSAieZM7YWVOdo5RyYg3kY1QJoxviIlzSIqimIh0uYh2eJdyyZZ76Yh4yYl46YmlB42BiYjnIZgGY5Z/OYqhuJh22Zh9CZh9iZipN5eQWYgvWZiV+JidaJgLpZeYaZmduZmHeYmhyZen6ZeRyZijuYmsqZmuyZmwWZelCZrARJiT6ZnlQZkQYSi8yXqZOZlGOJzE+ThDWJzImZxuEhAAOw==)

Imagine Alice has an asset _A_ and Bob has an asset _B_. They would like to trade their assets but neither wants to send their asset first. After all, what if the other party does not hold up their end of the trade and runs away with both assets? A deadlock will be reached where no party wants to send their asset first.

The traditional way to solve this problem is to introduce a third party _C_ which both _A_ and _B_ trust. _A_ or _B_ can now go first and send their asset to _C_. _C_ then waits for the other party to send their asset and only then does _C_ release both assets.

The blockchain way is to replace the trusted third party _C_ with code on a blockchain, specifically a smart contract that verifiably acts the same way a trusted third party would. A smart contract is superior to a trusted third party because of a number of reasons e.g. can you be sure that the trusted third party isn't colluding with the person on the other side of the trade? You can be sure with a smart contract because you can look at the code before running it.

I'll end this background section here. The internet already has a lot of material on escrows on blockchains. Let's now look at how to build such an escrow on Solana.

[#](#building-the-escrow-program-alice-s-transaction) Building the escrow program - Alice's Transaction
-------------------------------------------------------------------------------------------------------

### [#](#setting-up-the-project) Setting up the project

Head over to the [template repo (opens new window)](https://github.com/mvines/solana-bpf-program-template), click `Use this template`, and set up a repo. The Solana ecosystem is still young so this is what we've got for now. Vscode with the Rust extension is what I use. You'll also need [`Rust` (opens new window)](https://www.rust-lang.org/tools/install). Additionally, go [here (opens new window)](https://docs.solana.com/cli/install-solana-cli-tools) to install the Solana dev tools. (If you're on mac and there are no binaries for the version you want, follow the "build from source" section and add installed the bins to path. If it doesn't build because a command cannot be found, try installing [_coreutils_ (opens new window)](https://formulae.brew.sh/formula/coreutils) and [_binutils_ (opens new window)](https://formulae.brew.sh/formula/binutils) with homebrew).

If you don't know how to test solana programs yet, remove all the testing code. Testing programs is a topic for another blog post. Remove the testing code in `lib.rs` as well as the `tests` folder next to `src`. Lastly, remove the testing dependencies from [`Cargo.toml` (opens new window)](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html?highlight=cargo#creating-a-project-with-cargo). It should now look like this:

    [package]
    name = "solana-escrow"
    version = "0.1.0"
    edition = "2021"
    license = "WTFPL"
    publish = false
    
    [dependencies]
    solana-program = "1.9.4"
    
    [lib]
    crate-type = ["cdylib", "lib"]
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  

### [#](#entrypoint-rs-programs-and-accounts) entrypoint.rs, programs, and accounts

Have a look into `lib.rs`. First, the required [crates (opens new window)](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) are brought into scope using [use (opens new window)](https://doc.rust-lang.org/stable/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html). Then we use the `entrypoint!` [macro (opens new window)](https://doc.rust-lang.org/stable/book/ch19-06-macros.html) to declare the `process_instruction` function the [entrypoint (opens new window)](https://docs.solana.com/developing/on-chain-programs/developing-rust#program-entrypoint) to the program. Entrypoints are the only way to call a program; all calls go through the function declared as the entrypoint.

> When called, a program is passed to its [BPF Loader (opens new window)](https://docs.solana.com/developing/on-chain-programs/overview#loaders) which processes the call. Different BPF loaders may require different entrypoints.

The reason for the existence of multiple BPF Loaders is that it itself is a program. If updates are made to the program, a new program version has to be deployed. We can see that the BPF loader we are using requires the entrypoint function to take 3 arguments. `program_id` is simply the program id of the currently executing program. Why you'd want access to it inside the program will become apparent later. `intruction_data` is data passed to the program by the caller, it could be anything. Finally, to understand what `accounts` are, we have to dive deeper into the [solana programming model (opens new window)](https://docs.solana.com/developing/programming-model/overview). The reason we need accounts is because

> Solana programs are stateless

If you want to store state, use [accounts (opens new window)](https://docs.solana.com/developing/programming-model/accounts). Programs themselves are stored in accounts which are marked `executable`. Each account can hold data and [SOL (opens new window)](https://docs.solana.com/terminology#sol). Each account also has an `owner` and only the owner may debit the account and adjust its data. Crediting may be done by anyone. Here's an example of an [account (opens new window)](https://explorer.solana.com/address/6TkKqq15wXjqEjNg9zqTKADwuVATR9dW3rkNnsYme1ea). As you can see in the example account, it has its owner field set to the `System Program`. As a matter of fact,

> Accounts can only be owned by programs

Now you might be thinking "does that mean that my own SOL account is actually not owned by myself?". And you'd be right! But fear not, your funds are [safu (opens new window)](https://www.urbandictionary.com/define.php?term=Safu). The way it works is that even basic SOL transactions are handled by a program on Solana: the `system program`. (As a matter of fact, even programs are owned by programs. Remember, they are stored in accounts and these `executable` accounts are owned by the BPF Loader. The only programs not owned by the BPF loader are - of course - the BPF loader itself and the System Program. They are owned by the NativeLoader and have special privileges such as allocating memory or marking accounts as executable)

![](/assets/img/always-has-been.137c6342.jpeg)

[If you look at the system program (opens new window)](https://github.com/solana-labs/solana/blob/73e6038986d6fff6efced6378cfcd57cb34c220c/runtime/src/system_instruction_processor.rs#L222) you'll see that although the program owns all basic SOL accounts, it can only transfer SOL from an account when the transaction has been signed by the private key of the SOL account being debited.

> In theory, programs have full autonomy over the accounts they own. It is up to the program's creator to limit this autonomy and up to the users of the program to verify the program's creator has really done so

We'll get to how a program can check whether a transaction has been signed and how a program becomes the owner of an account in a bit. Before we conclude the entrypoint section, there is one more thing to know.

> All accounts to be read or written to must be passed into the entrypoint function

This allows the runtime to parallelise transactions. If the runtime knows all the accounts that will be written to and read by everyone at all times it can run those transactions in parallel that do not touch the same accounts or touch the same accounts but only read and don't write. If a transaction violates this constraint and reads or writes to an account of which the runtime has not been notified, the transaction will fail.

Now, to finally conclude this section, create a new `entrypoint.rs` file next to `lib.rs` and move the `lib.rs` code there. Finally, register the entrypoint module inside `lib.rs`. **You will have to do this for all files we create**.

    // inside lib.rs, only the following line should be in here
    pub mod entrypoint;
    

1  
2  

theory recap 📚

*   each program is processed by its BPF Loader and has an entrypoint whose structure depends on which BPF Loader is used
*   accounts are used to store state
*   accounts are owned by programs
*   only the account owner may debit an account and adjust its data
*   all accounts to be written to or read must be passed into the entrypoint

### [#](#instruction-rs-part-1-general-code-structure-and-the-beginning-of-the-escrow-program-flow) instruction.rs Part 1, general code structure, and the beginning of the escrow program flow

#### [#](#code-structure) code structure

Next, create a file `instruction.rs` next to the other two and register it inside `lib.rs` like you did with the entrypoint. To understand the new file's purpose, let's look at a common way to structure a program's code and the way we will structure our program as well.

    .
    ├─ src
    │  ├─ lib.rs -> registering modules
    │  ├─ entrypoint.rs -> entrypoint to the program
    │  ├─ instruction.rs -> program API, (de)serializing instruction data
    │  ├─ processor.rs -> program logic
    │  ├─ state.rs -> program objects, (de)serializing state
    │  ├─ error.rs -> program specific errors
    ├─ .gitignore
    ├─ Cargo.lock
    ├─ Cargo.toml
    ├─ Xargo.toml
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  

The flow of a program using this structure looks like this:

1.  Someone calls the entrypoint
2.  The entrypoint forwards the arguments to the processor
3.  The processor asks `instruction.rs` to decode the `instruction_data` argument from the entrypoint function.
4.  Using the decoded data, the processor will now decide which processing function to use to process the request.
5.  The processor may use `state.rs` to encode state into or decode the state of an account which has been passed into the entrypoint.

As you can see,

> instruction.rs defines the "API" of a program

While there is only one entrypoint, program execution can follow different paths depending on the given instruction data that is decoded inside `instruction.rs`.

#### [#](#beginning-of-the-escrow-program-flow) beginning of the escrow program flow

Let's now look at the different execution paths our program may take by zooming out and sketching the program flow for our escrow program.

Remember we have two parties _Alice_ and _Bob_ which means there are two `system_program` accounts. Because _Alice_ and _Bob_ want to transfer tokens, we'll make use of - you guessed it! - the `token program`. In the token program, to hold a token, you need a token account. Both _Alice_ and _Bob_ need an account for each token. So we get 4 more accounts. I'll call our tokens X and token Y, Alice gets an X and a Y account and so does Bob. (Our own tokens are X and Y but this escrow will work for any tokens on solana, like [USDC (opens new window)](https://www.coingecko.com/en/coins/usd-coin) and [SRM (opens new window)](https://www.coingecko.com/en/coins/serum)). Since escrow creation and the trade won't happen inside a single transaction, it's probably a good idea to have another account to save some escrow data (it will e.g. store how much of token Y Alice wants in exchange for her token X, but we'll get to that later!). Note that this account is created for each exchange. For now, our world looks like this:

![](/assets/img/escrow-sketch-1.6df070a8.png)

Now there are two questions you might ask yourself. How will Alice and Bob transfer ownership of X and Y to the escrow respectively and how are their main accounts connected to their token accounts? To find answers to these questions, we must briefly jump into the `token program`.

### [#](#the-token-program-part-1) The token program Part 1

#### [#](#token-ownership) token ownership

The naive way one might connect Alice's main account to her token accounts is by not connecting them at all. Whenever she wanted to transfer a token, she'd use the private key of the token account. Clearly, this would not be sustainable if Alice owned many tokens because that would require her to keep a private key for each token account.

It would be much easier for Alice if she just had one private key for all her token accounts and this is exactly how the token program does it! It assigns each token account an owner. Note that this token account owner attribute is **not** the same as the account owner. The account owner is an internal Solana attribute that will always be a program. The new token owner attribute is something the token program declares in user space (i.e. in the program they are building). It's encoded inside a token account's `data`, in addition to [other properties (opens new window)](https://github.com/solana-labs/solana-program-library/blob/80e29ef6b9a081d457849a2ca42db50d7da0e37e/token/program/src/state.rs#L86) such as the balance of tokens the account holds.I will call the token owner "authority" and the solana internal owner "owner" in the rest of this post to avoid confusion. The existence of the authority also means that once a token account has been set up, its private key is useless, only its authority matters. And the authority is going to be some other address, in our case Alice's and Bob's main account addresses respectively. When making a token transfer they simply have to sign the tx (tx=transaction) with the private key of their main account.

> All internal Solana internal account information are saved into [fields on the account (opens new window)](https://docs.rs/solana-program/1.5.0/solana_program/account_info/struct.AccountInfo.html#fields) but never into the `data` field which is solely meant for user space information

![](/assets/img/account.7425ef71.png)

We can see all this when looking at a token account in the [explorer (opens new window)](https://explorer.solana.com/address/EzMCP3PVVkZD4YFXhJNKhgwMihuxxeFqH4F6uPxFxFAe). It parses the account's `data` property and displays its user space fields formatted properly.

You've probably noticed the `mint` field in the explorer. This is how we know which token the token account belongs to. For each token there is 1 mint account that holds the token's metadata such as the supply. We'll need this field later to verify that the token accounts Alice and Bob use really belong to asset X and Y and that neither party is sneaking in a wrong asset.

With all this in mind, we can populate our world with more information:

![](/assets/img/escrow_token_accounts_1.ae89bb8d.png)

For simplicity's sake I have removed the programs and their arrows and simply colored the other accounts appropriately. Once again, the "owned by" arrows in the diagram are referring to the user space ownership, not the internal solana account ownership.

Now we know how all those accounts are connected but we don't know yet how Alice can transfer tokens to the escrow. We'll cover this now.

#### [#](#transferring-ownership) transferring ownership

The only way to own units of a token is to own a token account that holds some token balance of the token referenced by the account's (user space) `mint` property. Hence, the escrow program will need an account to hold Alice's X tokens. One way of achieving this is to have Alice create a temporary X token account to which she transfers the X tokens she wants to trade (The token program sets no limit on how many token accounts for the same mint one may be the authority of). Then, using a function in the token program, she transfers the authority of the temporary X token account to the escrow program. Let's add the temporary account to our escrow world. The image shows the escrow world _before_ Alice transfers token account ownership.

![](/assets/img/escrow_token_accounts_2.9291f5c8.png)

There is one more problem here. What exactly does Alice transfer ownership to? Enter [_Program Derived Addresses_ (opens new window)](https://docs.solana.com/developing/programming-model/calling-between-programs#program-derived-addresses).

theory recap 📚

*   developers should use the `data` field to save data inside accounts
*   the token program owns token accounts which - inside their `data` field - hold [relevant information (opens new window)](https://github.com/solana-labs/solana-program-library/blob/master/token/program/src/state.rs#L86)
*   the token program also owns token mint accounts with [relevant data (opens new window)](https://github.com/solana-labs/solana-program-library/blob/master/token/program/src/state.rs#L86)
*   each token account holds a reference to their token mint account, thereby stating which token mint they belong to
*   the token program allows the authority of a token account to transfer its ownership to another address
*   All internal Solana internal account information are saved into [fields on the account (opens new window)](https://docs.rs/solana-program/1.5.0/solana_program/account_info/struct.AccountInfo.html#fields) but never into the data field which is solely meant for user space information

### [#](#program-derived-addresses-pdas-part-1) Program Derived Addresses (PDAs) Part 1

We'd like some way for the program to own the X tokens while the escrow is open and waiting for Bob's transaction. The question is then, can programs be made the authority of a token account?

![](/assets/img/no_but_yes.f87dbd0e.jpg)

The trick is to assign the token account authority to a _Program Derived Address_ (PDA) of the escrow program. For now, it is enough for you to know this address exists and we can use it to let a program sign transactions and make it the authority of token accounts. We will cover PDAs in depth later but for now let's go back to coding!

### [#](#instruction-rs-part-2) instruction.rs Part 2

We left off at `instruction.rs` with the knowledge that this file would define the API of the program but without having written any code yet. Let's start coding by adding an `InitEscrow` API endpoint.

    // inside instruction.rs
    pub enum EscrowInstruction {
    
        /// Starts the trade by creating and populating an escrow account and transferring ownership of the given temp token account to the PDA
        ///
        ///
        /// Accounts expected:
        ///
        /// 0. `[signer]` The account of the person initializing the escrow
        /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
        /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
        /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade.
        /// 4. `[]` The rent sysvar
        /// 5. `[]` The token program
        InitEscrow {
            /// The amount party A expects to receive of token Y
            amount: u64
        }
    }
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  
13  
14  
15  
16  
17  
18  
19  

Although `instruction.rs` does not touch accounts, it is helpful to define which accounts you expect here so all the required calling info is in one place and easy to find for others. Additionally, it's helpful to add required account properties inside brackets. Note that everything after the `///` is a comment an has no effect on the program, it's only there for documentation purposes. The `writable` property should remind you of the parallelisation I explained above. If the caller does not mark the account `writable` in their calling code but the program attempts to write to it, the transaction will fail.

Let me explain why the endpoint looks like it does:

    0. `[signer]` The account of the person initializing the escrow
    

1  

We need Account 0 and specifically Account 0 as a signer because transferring the ownership of the temporary account requires Alice's signature. I'll be referring to Alice as the _initializer_ and Bob as the _taker_ in the code (Alice inits the escrow, Bob takes the trade. Pls let me know if you can come up with better naming)

    1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
    

1  

Account 1 is the temp token X account which needs to be writable. This is because changing token account ownership is a user space change which means the `data` field of the account will be changed

    2. `[]` The initializer's token account for the token they will receive should the trade go through
    

1  

Account 2 is Alice's token Y account. While it will be written to eventually, it won't happen in this transaction which is why we can leave the brackets empty (implying read-only)

    3. `[writable]` The escrow account, it will hold all necessary info about the trade.
    

1  

Account 3 is the escrow account which also needs to be writable because the program will write the escrow information into it

    4. `[]` The rent sysvar
    

1  

Account 4 is the `Rent` sysvar. I'll explain this in detail once we get to writing the `processor` code.

    5. `[]` The token program
    

1  

What you should remember for now is that Account 5 is the account of the token program itself. I will explain why we need this account as well when we get to writing the `processor` code.

> Solana has sysvars that are parameters of the Solana cluster you are on. These sysvars can be accessed through accounts and store parameters such as what the current fee or rent is. As of `solana-program` version `1.6.5`, [sysvars can also be accessed without being passed into the entrypoint as an account (opens new window)](https://github.com/solana-labs/solana/blob/a1a0d6f84b30ecea1fd1b699aa3cd6ab2741db77/programs/bpf/rust/sysvar/src/lib.rs) (this tutorial will continue to use the old way for now, but you shouldn't!).

    InitEscrow {
        /// The amount party A expects to receive of token Y
        amount: u64
    }
    

1  
2  
3  
4  

Finally, the program requires the amount of token Y that Alice wants to receive for her X tokens. This amount is not provided through an account but through the `instruction_data`.

`instruction.rs` is responsible for decoding `instruction_data` so that's that we'll do next.

  

  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  

  

    // inside instruction.rs
    use std::convert::TryInto;
    use solana_program::program_error::ProgramError;
    
    use crate::error::EscrowError::InvalidInstruction;
    
     pub enum EscrowInstruction {
    
        /// Starts the trade by creating and populating an escrow account and transferring ownership of the given temp token account to the PDA
        ///
        ///
        /// Accounts expected:
        ///
        /// 0. `[signer]` The account of the person initializing the escrow
        /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
        /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
        /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade.
        /// 4. `[]` The rent sysvar
        /// 5. `[]` The token program
        InitEscrow {
            /// The amount party A expects to receive of token Y
            amount: u64
        }
    }
    
    impl EscrowInstruction {
        /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
        pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
            let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
    
            Ok(match tag {
                0 => Self::InitEscrow {
                    amount: Self::unpack_amount(rest)?,
                },
                _ => return Err(InvalidInstruction.into()),
            })
        }
    
        fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
            let amount = input
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(InvalidInstruction)?;
            Ok(amount)
        }
    }
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  
13  
14  
15  
16  
17  
18  
19  
20  
21  
22  
23  
24  
25  
26  
27  
28  
29  
30  
31  
32  
33  
34  
35  
36  
37  
38  
39  
40  
41  
42  
43  
44  
45  
46  
47  

`unpack` expects a [reference (opens new window)](https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html) to a slice of `u8`. It looks at the first byte (=`tag`) to determine how to decode (using [`match` (opens new window)](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)) the rest (=`rest`) of the slice. For now, we'll leave it at one instruction (ignoring the instruction where Bob takes the trade). `unpack_amount` decodes the `rest` to get a `u64` representing the `amount`. You can look up the individual functions yourself. What's most important for now is that you understand what is going on at a high level in the unpack function: 1. choose which instruction to build 2. build and return that instruction.

This won't compile because we are using an undefined error. Let's add that error next.

theory recap 📚

*   Solana has sysvars that are parameters of the Solana cluster you are on. These sysvars can be accessed through accounts and store parameters such as what the current fee or rent is. As of `solana-program` version `1.6.5`, [sysvars can also be accessed without being passed into the entrypoint as an account (opens new window)](https://github.com/solana-labs/solana/blob/a1a0d6f84b30ecea1fd1b699aa3cd6ab2741db77/programs/bpf/rust/sysvar/src/lib.rs) (this tutorial will continue to use the old way for now, but you shouldn't!).

### [#](#error-rs) error.rs

Create a new file `error.rs` next to the others and register it inside `lib.rs`. Then, add the following dependency to your `Cargo.toml`

  
  
  

  

    ...
    [dependencies]
    solana-program = "1.9.4"
    thiserror = "1.0.24"
    

1  
2  
3  
4  

and the following code to `error.rs`.

    // inside error.rs
    use thiserror::Error;
    
    #[derive(Error, Debug, Copy, Clone)]
    pub enum EscrowError {
        /// Invalid instruction
        #[error("Invalid Instruction")]
        InvalidInstruction,
    }
    

1  
2  
3  
4  
5  
6  
7  
8  
9  

What we are doing here is [defining an error type (opens new window)](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html). Instead of having to write the `fmt::Display` implementation ourselves as is done in the example the link points to, we use the handy [thiserror (opens new window)](https://docs.rs/thiserror/latest/thiserror/) library that does it for us using the [`#[error("..")]` (opens new window)](https://doc.rust-lang.org/book/ch19-06-macros.html?highlight=derive#how-to-write-a-custom-derive-macro) notation. This will become especially useful when we add more errors later on.

Looking back into `instruction.rs`, we can see that we are not quite done. The compiler is telling us it has no way of turning an EscrowError into a ProgramError ("the trait `std::convert::From<error::EscrowError>` is not implemented for `solana_program::program_error::ProgramError`"). So let's implement a way.

  
  
  

  
  
  
  
  
  
  
  

  

    // inside error.rs
    use thiserror::Error;
    
    use solana_program::program_error::ProgramError;
    
    #[derive(Error, Debug, Copy, Clone)]
    pub enum EscrowError {
        /// Invalid instruction
        #[error("Invalid Instruction")]
        InvalidInstruction,
    }
    
    impl From<EscrowError> for ProgramError {
        fn from(e: EscrowError) -> Self {
            ProgramError::Custom(e as u32)
        }
    }
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  
13  
14  
15  
16  
17  

Let's stop for a moment to understand what is happening here. We are implementing a _generic trait_, specifically the [`From` (opens new window)](https://doc.rust-lang.org/std/convert/trait.From.html) trait which the `?` operator wants to use. To implement this trait we have to implement the `from` function which carries out the conversion. The `ProgramError` enum provides the `Custom` variant that allows us to convert from our program's `EscrowError` to a `ProgramError`.

The reason we do this conversion in the first place is that the entrypoint returns a `Result` of either nothing or a `ProgramError`.

### [#](#processor-rs-part-1-rent-part-1-starting-to-process-the-initescrow-instruction) processor.rs Part 1, Rent Part 1, starting to process the InitEscrow instruction

#### [#](#pub-fn-process) pub fn process

After creating the entrypoint, an InitEscrow endpoint, and our first error, we can finally move on to code `processor.rs`. This is where the magic happens. Start by creating `processor.rs` and registering it inside `lib.rs`. Then paste the following into `processor.rs`.

    use solana_program::{
        account_info::AccountInfo,
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
    };
    
    use crate::instruction::EscrowInstruction;
    
    pub struct Processor;
    impl Processor {
        pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
            let instruction = EscrowInstruction::unpack(instruction_data)?;
    
            match instruction {
                EscrowInstruction::InitEscrow { amount } => {
                    msg!("Instruction: InitEscrow");
                    Self::process_init_escrow(accounts, amount, program_id)
                }
            }
        }
    }
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  
13  
14  
15  
16  
17  
18  
19  
20  
21  
22  

Let's start unpacking what's happening. First, we pass the reference to the slice holding the `instruction_data` from `entrypoint.rs` into the `unpack` function we created earlier ([Note the `?` after the function call (opens new window)](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html?highlight=question,mark#a-shortcut-for-propagating-errors-the--operator)). We use `match` to figure out which processing function to call. Trivial, for now. `msg!` logs where we are going.

#### [#](#fn-process-init-escrow) fn process\_init\_escrow

    use solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        program_error::ProgramError,
        msg,
        pubkey::Pubkey,
    };
    ...
    impl Processor {
        pub fn process{...}
        
        fn process_init_escrow(
            accounts: &[AccountInfo],
            amount: u64,
            program_id: &Pubkey,
        ) -> ProgramResult {
            let account_info_iter = &mut accounts.iter();
            let initializer = next_account_info(account_info_iter)?;
    
            if !initializer.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }
    
            Ok(())
        }
    }
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  
13  
14  
15  
16  
17  
18  
19  
20  
21  
22  
23  
24  
25  
26  

`process_init_escrow` is next. To be clear, `...` just means there is other stuff around the code you're seeing, don't copy those! Copy and replace your current `use solana_program...` with the one here or add the invididual missing pieces.

Inside `process_init_escrow` we first create an iterator of accounts. It needs to be mutable so we can take elements out of it. The first account we expect - as defined in `instruction.rs` - is the escrow's initializer, i.e. Alice's main account. She needs to be a signer which we check right away. It's just a boolean field on `AccountInfo`.

  
  
  
  
  
  
  
  
  
  
  
  
  

  
  
  
  
  

    ...
    fn process_init_escrow(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;
    
        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    
        let temp_token_account = next_account_info(account_info_iter)?;
    
        let token_to_receive_account = next_account_info(account_info_iter)?;
        if *token_to_receive_account.owner != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
    
        Ok(())
    }
    ...
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  
13  
14  
15  
16  
17  
18  
19  
20  
21  
22  
23  

Next, add the highlighted lines. The temporary token account needs to be writable but there is no need to explicitly check this. The transaction will fail automatically should Alice not mark the account as writable.

You might ask yourself, "why do we check that the `token_to_receive_account` is actually owned by the token program but don't do the same for the `temp_token_account`?". The answer is that later on in the function we will ask the token program to transfer ownership of the `temp_token_account` to the _PDA_. This transfer will fail if the `temp_token_account` is not owned by the token program, because - as I'm sure you remember - only programs that own accounts may change accounts. Hence, there is no need for us to add another check here.

We don't make any changes to the `token_to_receive_account` though (inside Alice's transaction). We will just save it into the escrow data so that when Bob takes the trade, the escrow will know where to send asset Y. Thus, for this account, we should add a check. Note that nothing terrible would happen if we didn't. Instead of Alice's transaction failing because of our added check, Bob's would fail because the token program will attempt to send the Y tokens to Alice but not be the authority of the `token_to_receive_account`. That said, it seems more reasonable to let the tx fail that actually led to the invalid state.

Finally, I'm sure you have noticed that we are using a crate here which we have not registered inside `Cargo.toml` yet. Let's do that now.

  
  
  

  

    [dependencies]
    solana-program = "1.9.4"
    thiserror = "1.0.24"
    spl-token = {version = "3.2.0", features = ["no-entrypoint"]}
    

1  
2  
3  
4  

We are using a slighly different way to import a dependency here than we did we the other dependencies. That's because we are importing another Solana program that has its own entrypoint. But our program should only have one entrypoint, the one we defined earlier. Luckily, the token program provides a switch to turn its entrypoint off with the help of a [cargo feature (opens new window)](https://doc.rust-lang.org/cargo/reference/features.html). We should define this feature in our program as well so others can import our program! I'll leave this to you with some hints: Check out the [token program's (opens new window)](https://github.com/solana-labs/solana-program-library/tree/master/token/program) `Cargo.toml` and its `lib.rs`. If you cannot or don't want to figure it out on your own, you can take a look into the escrow program I created.

Now back to `processor.rs`. Copy and replace the `solana_program` use statement and add more code to `process_init_escrow`:

  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  

  
  
  
  

    use solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        program_error::ProgramError,
        msg,
        pubkey::Pubkey,
        program_pack::{Pack, IsInitialized},
        sysvar::{rent::Rent, Sysvar},
    };
    //inside process_init_escrow
    ...
    let temp_token_account = next_account_info(account_info_iter)?;
    
    let token_to_receive_account = next_account_info(account_info_iter)?;
    if *token_to_receive_account.owner != spl_token::id() {
        return Err(ProgramError::IncorrectProgramId);
    }
    
    let escrow_account = next_account_info(account_info_iter)?;
    let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
    
    if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
        return Err(EscrowError::NotRentExempt.into());
    }
    
    let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.try_borrow_data()?)?;
    if escrow_info.is_initialized() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    
    Ok(())
    ...
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  
13  
14  
15  
16  
17  
18  
19  
20  
21  
22  
23  
24  
25  
26  
27  
28  
29  
30  
31  
32  

Now we see the [`Rent` (opens new window)](https://docs.solana.com/implemented-proposals/rent) sysvar in action. Let me explain:

> Rent is deducted from an account's balance according to their space requirements (i.e. the space an account and its fields take up in memory) regularly. An account can, however, be made rent-exempt if its balance is higher than some threshold that depends on the space it's consuming

Most of the time, you want your accounts to be rent-exempt, cause once their balance goes to zero, they _disappear_. More on this at the end of Bob's transaction.

Also, make sure to add the new error variant inside `error.rs` and adjust the use statement:

From

    use crate::instruction::EscrowInstruction;
    

1  

To

    use crate::{instruction::EscrowInstruction, error::EscrowError};
    

1  

Another unfamiliar thing is happening here. For the first time, we are accessing the `data` field. Because `data` is also just an array of `u8`, we need to deserialize it with `Escrow::unpack_unchecked`. This is a function inside `state.rs` which we'll create in the next section.

theory recap 📚

*   Rent is deducted from an account's balance according to their space requirements regularly. An account can, however, be made rent-exempt if its balance is higher than some threshold that depends on the space it's consuming

### [#](#state-rs) state.rs

Create `state.rs` and register it inside `lib.rs`. The state file is responsible for 1) defining state objects that the processor can use 2) serializing and deserializing such objects from and into arrays of `u8` respectively.

Start by adding the following to `state.rs`.

    use solana_program::pubkey::Pubkey;
    
    pub struct Escrow {
        pub is_initialized: bool,
        pub initializer_pubkey: Pubkey,
        pub temp_token_account_pubkey: Pubkey,
        pub initializer_token_to_receive_account_pubkey: Pubkey,
        pub expected_amount: u64,
    }
    

1  
2  
3  
4  
5  
6  
7  
8  
9  

We need to save `temp_token_account_pubkey` so that when Bob takes the trade, the escrow program can send tokens from the account at `temp_token_account_pubkey` to Bob's account. We already know that Bob will have to pass in the account into his entrypoint call eventually so why do we save it here? First, if we save its public key here, Bob can easily find the address of the accounts he needs to pass into the entrypoint. Otherwise Alice would have to send him not only the escrow acount address but also all her account addresses. Secondly, and more important for security is that Bob could pass in a different token account. Nothing stops him from doing so if we don't add a check requiring him to pass in the account with `temp_token_account_pubkey` as its public key. And to add that check later in the processor, we need the InitEscrow instruction to save the `temp_token_account_pubkey`.

> When writing Solana programs, be mindful of the fact that any accounts may be passed into the entrypoint, including different ones than those defined in the API inside `instruction.rs`. It's the program's responsibility to check that `received accounts == expected accounts`

`initializer_token_to_receive_account_pubkey` needs to be saved so that when Bob takes the trade, his tokens can be sent to that account. `expected_amount` will be used to check that Bob sends enough of his token. That leaves `initializer_pubkey` and `is_initialized`. I'll explain the latter now and the former later on.

We use `is_initialized` to determine whether a given escrow account is already in use. This, serialization, and deserialization are all standardized in the [traits (opens new window)](https://doc.rust-lang.org/book/ch10-02-traits.html) of the [`program pack` module (opens new window)](https://docs.rs/solana-program/latest/solana_program/program_pack/index.html). First, implement `Sealed` and `IsInitialized`.

    // inside state.rs
    use solana_program::{
        program_pack::{IsInitialized, Pack, Sealed},
        pubkey::Pubkey,
    };
    ...
    impl Sealed for Escrow {}
    
    impl IsInitialized for Escrow {
        fn is_initialized(&self) -> bool {
            self.is_initialized
        }
    }
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  
13  

`Sealed` is just Solana's version of Rust's `Sized` trait although there does not seem to be any difference between the two. Now, `Pack`, which relies on `Sealed` and in our case also on `IsInitialized` being implemented. It's a big but simple block of code. I'll split it into 2 parts. Let's start with the first one (you can copy and replace the `use` imports again):

    // inside state.rs
    use solana_program::{
        program_pack::{IsInitialized, Pack, Sealed},
        program_error::ProgramError,
        pubkey::Pubkey,
    };
    
    use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
    ...
    impl Pack for Escrow {
        const LEN: usize = 105;
        fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
            let src = array_ref![src, 0, Escrow::LEN];
            let (
                is_initialized,
                initializer_pubkey,
                temp_token_account_pubkey,
                initializer_token_to_receive_account_pubkey,
                expected_amount,
            ) = array_refs![src, 1, 32, 32, 32, 8];
            let is_initialized = match is_initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            };
    
            Ok(Escrow {
                is_initialized,
                initializer_pubkey: Pubkey::new_from_array(*initializer_pubkey),
                temp_token_account_pubkey: Pubkey::new_from_array(*temp_token_account_pubkey),
                initializer_token_to_receive_account_pubkey: Pubkey::new_from_array(*initializer_token_to_receive_account_pubkey),
                expected_amount: u64::from_le_bytes(*expected_amount),
            })
        }
    }
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  
13  
14  
15  
16  
17  
18  
19  
20  
21  
22  
23  
24  
25  
26  
27  
28  
29  
30  
31  
32  
33  
34  
35  

The first requirement for something implementing `Pack` is defining `LEN` which is the size of our type. Looking at our Escrow struct, we can see how to calculate the length of the struct by adding the sizes of the individual data types: `1 (bool) + 3 * 32 (Pubkey) + 1 * 8 (u64) = 105`. It's okay to use an entire `u8` for the bool since it'll make our coding easier and the cost of those extra wasted bits is infinitesimal.

After defining the escrow's length, we implement `unpack_from_slice` which turns an array of `u8` into an instance of the Escrow struct we defined above. Nothing too interesting happens here. Notable here is the use of [arrayref (opens new window)](https://docs.rs/arrayref/latest/arrayref/), a library for getting references to sections of a slice. The docs should be enough to understand the (just 4) different functions from the library. Make sure to add the library to `Cargo.toml`.

    ...
    [dependencies]
    ...
    arrayref = "0.3.6"
    ...
    

1  
2  
3  
4  
5  

We can now deserialize state, serialization is next.

    ...
    impl Pack for Escrow {
        ...
        fn pack_into_slice(&self, dst: &mut [u8]) {
            let dst = array_mut_ref![dst, 0, Escrow::LEN];
            let (
                is_initialized_dst,
                initializer_pubkey_dst,
                temp_token_account_pubkey_dst,
                initializer_token_to_receive_account_pubkey_dst,
                expected_amount_dst,
            ) = mut_array_refs![dst, 1, 32, 32, 32, 8];
    
            let Escrow {
                is_initialized,
                initializer_pubkey,
                temp_token_account_pubkey,
                initializer_token_to_receive_account_pubkey,
                expected_amount,
            } = self;
    
            is_initialized_dst[0] = *is_initialized as u8;
            initializer_pubkey_dst.copy_from_slice(initializer_pubkey.as_ref());
            temp_token_account_pubkey_dst.copy_from_slice(temp_token_account_pubkey.as_ref());
            initializer_token_to_receive_account_pubkey_dst.copy_from_slice(initializer_token_to_receive_account_pubkey.as_ref());
            *expected_amount_dst = expected_amount.to_le_bytes();
        }
    }
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  
13  
14  
15  
16  
17  
18  
19  
20  
21  
22  
23  
24  
25  
26  
27  
28  

This is pretty much the same as the `unpack_from_slice` function, just vice versa! This time, we also pass in `&self`. We didn't have to do this inside `unpack_from_slice` because _there was no self_ yet. `unpack_from_slice` was a static constructor function returning a new instance of an escrow struct. When we `pack_into_slice`, we already have an instance of an Escrow struct and now serialize it into the given `dst` slice. And that's it for `state.rs`! But wait, if we look back into `processor.rs`, we call `unpack_unchecked`, a function we didn't define, so where is it coming from? The answer is that traits can have default functions that may be overridden but don't have to be. [Look here (opens new window)](https://docs.rs/solana-program/latest/src/solana_program/program_pack.rs.html#29-39) to find out about `Pack`'s default functions.

With `state.rs` done, let's go back to the `processor.rs` and adjust one of our `use` statements.

From

    use crate::{instruction::EscrowInstruction, error::EscrowError};
    

1  

to

    use crate::{instruction::EscrowInstruction, error::EscrowError, state::Escrow};
    

1  

### [#](#processor-part-2-pdas-part-2-cpis-part-1) Processor Part 2, PDAs Part 2, CPIs Part 1

Let's finish the `process_init_escrow` function by first adding the state serialization. We've already created the escrow struct instance and checked that it is indeed uninitialized. Time to populate the struct's fields!

  
  
  
  
  
  
  

  

    // inside process_init_escrow
    ...
    let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.try_borrow_data()?)?;
    if escrow_info.is_initialized() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    
    escrow_info.is_initialized = true;
    escrow_info.initializer_pubkey = *initializer.key;
    escrow_info.temp_token_account_pubkey = *temp_token_account.key;
    escrow_info.initializer_token_to_receive_account_pubkey = *token_to_receive_account.key;
    escrow_info.expected_amount = amount;
    
    Escrow::pack(escrow_info, &mut escrow_account.try_borrow_mut_data()?)?;
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  
13  
14  

Pretty straightforward. `pack` is another default function which internally calls our `pack_into_slice` function.

#### [#](#pdas-part-2) PDAs Part 2

There's one thing left to do inside `process_init_escrow`: transferring (user space) ownership of the temporary token account to the PDA. This is a good time to explain what PDAs actually are and why we might need the `program_id` inside a process function. Copy and look at the higlighted line:

  
  
  
  

  

    // inside process_init_escrow
    ...
    escrow_info.expected_amount = amount;
    Escrow::pack(escrow_info, &mut escrow_account.try_borrow_mut_data()?)?;
    let (pda, _bump_seed) = Pubkey::find_program_address(&[b"escrow"], program_id);
    

1  
2  
3  
4  
5  

We create a PDA by passing in an array of seeds and the `program_id` into the `find_program_address` function. We get back a new `pda` and `bump_seed` (we won't need the bump seed in Alice's tx) with a 1/(2^255) chance the function fails ([2^255 is a BIG number (opens new window)](https://youtu.be/S9JGmA5_unY)). In our case the seeds can be static. There are cases such as in the [Associated Token Account program (opens new window)](https://github.com/solana-labs/solana-program-library/blob/596700b6b100902cde33db0f65ca123a6333fa58/associated-token-account/program/src/lib.rs#L24) where they aren't (cause different users should own different associated token accounts). We just need `1` PDA that can own `N` temporary token accounts for different escrows occuring at any and possibly the same point in time.

Ok, but what _is_ a PDA? Normally, Solana key pairs use the [`ed25519` (opens new window)](http://ed25519.cr.yp.to/) standard. This means normal public keys lie on the `ed25519` elliptic curve. PDAs are public keys that are derived from the `program_id` and the seeds as well as _having been pushed off the curve by the bump seed_. Hence,

> Program Derived Addresses do not lie on the `ed25519` curve and therefore have no private key associated with them.

![](/assets/img/this_is_pda.8b81f953.jpg)

A PDA is just a random array of bytes with the only defining feature being that they are _not_ on that curve. That said, they can still be used as normal addresses most of the time. You should absolutely read the two different docs on PDAs ([here (opens new window)](https://docs.solana.com/developing/programming-model/calling-between-programs#program-derived-addresses) and [here(find\_program\_address calls this function) (opens new window)](https://docs.rs/solana-program/latest/src/solana_program/pubkey.rs.html#114)). We don't use the bump seed here yet (also indicated by the underscore before the variable name). We will do that when we look into how it's possible to sign messages with PDAs even without a private key in PDAs Part 3 inside Bob's transaction.

#### [#](#cpis-part-1) CPIs Part 1

For now, let's look at how we can transfer the (user space) ownership of the temporary token account to the PDA. To do this, we will call the token program from our escrow program. This is called a [_Cross-Program Invocation_ (opens new window)](https://docs.solana.com/developing/programming-model/calling-between-programs#cross-program-invocations) and executed using either the `invoke` or the `invoke_signed` function. Here we use `invoke`. In Bob's transaction we will use `invoke_signed`. The difference will become clear then. `invoke` takes two arguments: an instruction and an array of accounts.

    use solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        program_error::ProgramError,
        msg,
        pubkey::Pubkey,
        program_pack::{Pack, IsInitialized},
        sysvar::{rent::Rent, Sysvar},
        program::invoke
    };
    // inside process_init_escrow
    ...
    let token_program = next_account_info(account_info_iter)?;
    let owner_change_ix = spl_token::instruction::set_authority(
        token_program.key,
        temp_token_account.key,
        Some(&pda),
        spl_token::instruction::AuthorityType::AccountOwner,
        initializer.key,
        &[&initializer.key],
    )?;
    
    msg!("Calling the token program to transfer token account ownership...");
    invoke(
        &owner_change_ix,
        &[
            temp_token_account.clone(),
            initializer.clone(),
            token_program.clone(),
        ],
    )?;
    
    Ok(())
    // end of process_init_escrow
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  
13  
14  
15  
16  
17  
18  
19  
20  
21  
22  
23  
24  
25  
26  
27  
28  
29  
30  
31  
32  
33  
34  

Copy and replace the `solana_program` use statement. We continue with `process_init_escrow` by getting the token\_program account. It's a rule that the program being called through a CPI must be included as an account in the 2nd argument of `invoke` (and `invoke_signed`). Next, we create the instruction. This is just the instruction that the token program would expect were we executing a normal call. The token program defines some helper functions inside its `instruction.rs` that we can make use of. Of particular interest to us is the `set_authority` function which is a builder function to create such an instruction. We pass in the token program id, then the account whose authority we'd like to change, the account that's the new authority (in our case the PDA), the type of authority change (there are different authority types for token accounts, we care about changing the main authority), the current account authority (Alice -> initializer.key), and finally the public keys signing the CPI.

The concept that is being used here is [_Signature Extension_ (opens new window)](https://docs.solana.com/developing/programming-model/calling-between-programs#instructions-that-require-privileges). In short,

> When including a `signed` account in a program call, in all CPIs including that account made by that program inside the current instruction, the account will also be `signed`, i.e. the _signature is extended_ to the CPIs.

In our case this means that because Alice signed the `InitEscrow` transaction, the program can make the token program `set_authority` CPI and include her pubkey as a signer pubkey. This is necessary because changing a token account's authority should of course require the approval of the current authority.

Next to the instruction, we also need to pass in the accounts that are required by the instruction, in addition to the account of the program we are calling. You can look these up by going to the token programs `instruction.rs` and finding the setAuthority Enum whose comments will tell you which accounts are required (in our case, the current authority's account and the account whose authority is to be changed).

Note that before making a CPI, we should add another check that the `token_program` is truly the account of the token program. Otherwise, we might be calling a rogue program. If you're using the `spl-token` crate above version `3.1.1` (which I do in this guide), you don't have to do this if you use their instruction builder functions. They do it for you.

Finally, adjust `entrypoint.rs` so it looks like this:

    use solana_program::{
        account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey
    };
    
    use crate::processor::Processor;
    
    entrypoint!(process_instruction);
    fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        Processor::process(program_id, accounts, instruction_data)
    }
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  
13  
14  

Now you might be thinking "wait, we are giving the escrow program control over the temporary account but Alice never transferred anything into it! Shouldn't that have happened before?". And you're right, it should happen before! But importantly, it does not have to happen inside the escrow program itself. We'll look into how she can make the transfer in the same tx as the call to our program in the next section.

theory recap 📚

*   Program Derived Addresses do not lie on the `ed25519` curve and therefore have no private key associated with them.
*   When including a `signed` account in a program call, in all CPIs including that account made by that program inside the current instruction, the account will also be `signed`, i.e. the _signature is extended_ to the CPIs.

### [#](#trying-out-the-program-understanding-alice-s-transaction) Trying out the program, understanding Alice's transaction

Because we've built a part of the program that is complete in itself, we can now try it out! In doing so, we can acquire more knowledge about Solana, e.g. where do accounts come from?

I've prepared two ways for you to interact with the program and I encourage you to try them both. First, there is a set of scripts that test whether you implemented the program correctly. Reading them should also provide you with a better understanding of how we can interact with solana programs on the client side. You can find the scripts and instructions on how to use them [here (opens new window)](https://github.com/paul-schaaf/solana-escrow).

You can use also this UI to try out your program ([Here's (opens new window)](https://github.com/paul-schaaf/escrow-ui) the code). It's a little more complex than just executing the scripts but I explain how it works and what you need to do to make it work below. Feel free to build your own!

#### [#](#deploying-your-program-on-localnet) Deploying your program on localnet

First, use the `cargo build-bpf` command to compile your program to a file with the `so` file extension.

Run `solana-keygen new` to create and save a solana keypair locally. (or [create a cli wallet (opens new window)](https://docs.solana.com/wallet-guide/cli) of your choosing.)

Fire up your localnet with the command (that should now be in your PATH) `solana-test-validator`. When calling `solana config get`, your "RPC URL" should now equal `http://localhost:8899`. If not, run `solana config set --url http://localhost:8899`. Running `solana balance` will show your balance which should NOT be 0. If it is, stop the validators, make sure you have created a key with `solana-keygen new` and start it again from genesis with `solana-test-validator -r`.

Then, use the `solana deploy` command to deploy the program to localnet. The path to the program will have been printed by `cargo build-bpf`. (⚠️ you can only deploy the program locally for now because we are using functionality that is not enabled on any cluster yet)

    solana deploy PATH_TO_YOUR_PROGRAM
    

1  

The `deploy` command should print the program id which you can now paste into the UI above.

#### [#](#creating-a-throwaway-private-key) Creating a throwaway private key

My UI requires a private key (NEVER DO THIS IN A REAL APP). Go to [sollet.io (opens new window)](https://www.sollet.io) and create an entirely new account or a new account next to your main account. This account will represent Alice.

Change the sollet cluster to localnet.

After creating the wallet, airdrop yourself some SOL to pay for the tx fees. Then, click on your SOL account in the middle of the screen and then on `export` to export your private key byte array and paste it above.

Use your throwaway wallet for the next steps as well.

#### [#](#creating-tokens-for-testing-on-localnet) Creating tokens for testing on localnet

You'll also need a token to put into the escrow so head over to the [SPL Token UI (opens new window)](https://www.spl-token-ui.com). Make sure you choose _localnet_ here too.

During the next steps, you will create the token mint accounts for X and Y and 2 token accounts, Alice's X and Alice's Y account. After creating each token account (not the token mint account!), copy the account address and put it into the appropriate UI field. You could also write them down somewhere else so can reuse them when eventually testing the entire escrow, including Bob's transaction.

Start by heading over to `Create new token` inside the `Tokens` tab. Fill out the mint authority with your sollet pubkey and create the new token. Keep in mind, this is the _token mint account_ for token X, i.e. the account that holds all the metadata of the token e.g. its supply and who is allowed to mint it (if you set the mint authority correctly, that should be your sollet pubkey! You can verify this in the explorer).

Next, go to `Create account` inside the `Accounts` tab and fill in the address of the token you just created and use your sollet pubkey as the account owner. Create an account (doesn't matter whether it's associated or not). This is Alice's token X token account.

Then, go to `Edit account` inside the `Accounts` tab. The `mint` option is selected by default. Put in Alice's token X token account (the one you just created) as the destination account and some number in the amount field. Click `Mint to account`.

Go through the same steps for token Y. You don't have to mint tokens to Alice's token Y account.

#### [#](#creating-the-escrow) Creating the escrow

With all the steps completed, all that is left to do is to fill in Alice's expected amount and the amount she wants to put into the escrow. Fill in both numbers (the 2nd needs to be lower than what you minted to Alice's account) and hit `Init Escrow`.

#### [#](#understanding-what-just-happened-rent-part-2-and-commitment) Understanding what just happened, Rent Part 2, and Commitment

![](/images/escrow-alice-initial.jpg)

Previous Next

I've created a little slideshow to show the life of the transaction that Alice sends off. I've left out the internal and user space relationship arrows not to make the images too convoluted.

As you can see in the top right corner,

> there can be several _instructions_ (ix) inside one _transaction_ (tx) in Solana. These instructions are executed out _synchronously_ and the tx as a whole is executed _atomically_. These instructions can call different programs.

This means that if a single instruction fails, the entire transaction fails. Right in ix1, we can see how accounts come to life.

> The system program is responsible for allocating account space and assigning (internal - not user space) account ownership

Alice's transaction consists of 5 instructions.

    1. create empty account owned by token program
    2. initialize empty account as Alice's X token account
    3. transfer X tokens from Alice's main X token account to her temporary X token account
    4. create empty account owned by escrow program
    5. initialize empty account as escrow state and transfer temporary X token account ownership to PDA
    

1  
2  
3  
4  
5  

As you can see,

> instructions may depend on previous instructions inside the same transaction

I'll now walk you through the important parts of the frontend code which uses the Solana js/ts libraries. Feel free to look at [the code (opens new window)](https://github.com/paul-schaaf/escrow-ui/blob/master/src/util/initEscrow.ts) yourself.

    const tempTokenAccount = new Account();
    const createTempTokenAccountIx = SystemProgram.createAccount({
        programId: TOKEN_PROGRAM_ID,
        space: AccountLayout.span,
        lamports: await connection.getMinimumBalanceForRentExemption(AccountLayout.span, 'confirmed'),
        fromPubkey: feePayerAcc.publicKey,
        newAccountPubkey: tempTokenAccount.publicKey
    });
    

1  
2  
3  
4  
5  
6  
7  
8  

The first instruction that is created is to create the new X token account that will be transferred to the PDA eventually. Note that it's just built here, nothing is sent yet. The function requires the user to specify which program the new account should belong to (`programId`), how much space it should have (`space`), what the initial balance should be (`lamports`), where to transfer that balance from (`fromPubkey`) and the address of the new account (`newAccountPubkey`).

What about the `'confirmed'` argument? `confirmed` is one of the available [_Commitments_ (opens new window)](https://solana-labs.github.io/solana-web3.js/modules.html#Commitment) and tells us how to query the network. Which commitment level to pick depends on your use case. If you're moving millions and want to be as sure as possible that your tx cannot be rolled back, choose `finalized`. `confirmed` is still pretty safe because of [optimistic confirmation and slashing (opens new window)](https://docs.solana.com/proposals/optimistic-confirmation-and-slashing).

    const initTempAccountIx = Token.createInitAccountInstruction(TOKEN_PROGRAM_ID, XTokenMintAccountPubkey, tempTokenAccount.publicKey, feePayerAcc.publicKey);
    const transferXTokensToTempAccIx = Token
        .createTransferInstruction(TOKEN_PROGRAM_ID, initializerXTokenAccountPubkey, tempTokenAccount.publicKey, feePayerAcc.publicKey, [], amountXTokensToSendToEscrow);
    

1  
2  
3  

After building the ix for creating the new account, we call two functions provided by the [spl-token js library (opens new window)](https://www.npmjs.com/package/@solana/spl-token) to create the next two instructions. Nothing new here. Then, instruction 4 is creating another account, this time owned by the escrow program but still very similar to the first ix.

    const initEscrowIx = new TransactionInstruction({
        programId: escrowProgramId,
        keys: [
            { pubkey: initializerAccount.publicKey, isSigner: true, isWritable: false },
            { pubkey: tempTokenAccount.publicKey, isSigner: false, isWritable: true },
            { pubkey: new PublicKey(initializerReceivingTokenAccountPubkeyString), isSigner: false, isWritable: false },
            { pubkey: escrowAccount.publicKey, isSigner: false, isWritable: true },
            { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
            { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        ],
        data: Buffer.from(Uint8Array.of(0, ...new BN(expectedAmount).toArray("le", 8)))
    })
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  

The 5th and final ix - where we initiate the escrow - is more interesting since here we get almost no help from the Solana libraries. We manually create the instruction by calling its constructor (`new TransactionInstruction...`). The required format should feel familiar! It's exactly what our program entrypoint expects. We pass in the programId of our escrow program and then the keys. Here, we specify whether a given account will sign the tx - if it then doesn't the tx will fail - and whether an account is read-only - if it's then written to the tx will fail. Finally, we specify what will arrive at the entrypoint as `instruction_data`. We start with a `0` since the first byte is what we used in `instruction.rs` as a `tag` to determine how to decode the instruction. `0` means `InitEscrow`. The next bytes will be the `expected_amount`. We use the `bn.js` library to write our expected amount as an 8-byte array of little-endian numbers. 8 bytes because inside `instruction.rs` we decode a `u64` and little-endian because we decode it the slice with `u64::from_le_bytes`. We use a `u64` because that's the [max supply of a token (opens new window)](https://github.com/solana-labs/solana-program-library/blob/123a3dc1e43dbc6c90c503b2c27a0d9b264e9ede/token/program/src/state.rs#L22).

    const tx = new Transaction()
            .add(createTempTokenAccountIx, initTempAccountIx, transferXTokensToTempAccIx, createEscrowAccountIx, initEscrowIx);
    await connection.sendTransaction(tx, [initializerAccount, tempTokenAccount, escrowAccount], {skipPreflight: false, preflightCommitment: 'confirmed'});
    

1  
2  
3  

Finally, we create a new Transaction and add all the instructions. Then, we send off the tx with its signers. In the js library world, an `Account` has a double meaning and is also used as the object to hold a keypair. That means the signers we pass in include the private keys and can actually sign. Obviously, we have to add Alice's account as a signer - she pays the fees and needs to authorize transfers from her accounts. We also have to add the other two accounts because it turns out when the system program creates a new account, the tx needs to be signed by that account.

What we end up with after Alice's transaction is the last slide. There's a new esrow state account that holds relevant data to complete the trade as well as a new token account that is owned by a PDA of the escrow program. That token account's token balance is the amount of X tokens Alice would like to trade in for the expected amount (which is saved in the escrow state acount) of Y tokens.

An important note here is that while it's not important that all the instructions are in the same transaction, **it is important that at least ix 1,2 and ix 4,5 are in the same transaction**. This is because after an account has been created by the system program, it's kind of just floating on the blockchain, still uninitialized, with no authority. If, for example, you put ix 1 and 2 in different transactions, someone could try to send a tx between those two and initialize their own token account, using the then still ownerless account created by ix 1. This cannot happen if you put ix 1 and 2 in the same transaction since a tx is executed atomically.

#### [#](#adapting-the-frontend-for-real-life-use) Adapting the frontend for real life use

There are a couple of things that were left out - to keep things simple - but should definitely be added for a real program. First, the maximum token amount is U64\_MAX which is higher than javascript's number value. Hence, you need to find a way to handle this, either by limiting the allowed amount of tokens that can be put in or by accepting the token amount as a string and then using a library like `bn.js` to convert the string. Secondly, you should never have your users put in a private key. Use an external wallet like `solong` or the `sol-wallet-adapter` library. You'd create the transaction, add the instructions, and then ask whatever trusted service you're using to sign the transaction and send it back to you. You can then add the other two keypair accounts and send off the tx to the network.

theory recap 📚

*   There can be several _instructions_ (ix) inside one _transaction_ (tx) in Solana. These instructions are executed out _synchronously_ and the tx as a whole is executed _atomically_. These instructions can call different programs.
*   The system program is responsible for allocating account space and assigning (internal - not user space) account ownership
*   Instructions may depend on previous instructions inside the same transaction
*   Commitment settings give downstream developers ways to query the network which differ in finality likelihood

[#](#building-the-escrow-program-bob-s-transaction) Building the escrow program - Bob's Transaction
---------------------------------------------------------------------------------------------------

After Alice has created the escrow, she can send the escrow state account address to Bob. If he sends the expected amount of Y tokens to the escrow, the escrow will send him Alice's X tokens and Alice his Y tokens. I'll now show you how to make the escrow program ready for Bob's transaction. During this second part of the guide, you will also get to know some more Solana concepts! This part will be considerably shorter because we can reuse a lot of the code we wrote already and I won't spend much time on code that needs to be written but you already know how.

### [#](#instruction-rs-part-3-understanding-what-bob-s-transaction-should-do) instruction.rs Part 3, understanding what Bob's transaction should do

To understand what Bob's transaction should do, let's have a look once again at the state of things after Alice's transaction is complete.

![](/assets/img/escrow-alice-end.dd39a5e1.jpg)

We can see that there is an escrow account which holds all the info necessary for the trade between Alice and Bob and there also is a token account which holds Alice's X tokens and is owned by a PDA of the escrow program.What we (and Bob) would like the tx to do is move the X tokens from the PDA-owned X token account to his X token account. The escrow program should also subtract tokens from Bob's Y token account and add them to the Y token account Alice had the escrow program write into the escrow state account (the `initializer_token_to_receive_account_pubkey` property inside the Escrow struct inside `state.rs`). Lastly, the two accounts that have been created for the trade (the escrow state account and the temporary X token account) should be cleaned up since there is no need for them anymore.

Equipped with this knowledge, we can add the endpoint for what I've decided to call the `Exchange` instruction inside `instruction.rs`.

    /// Accepts a trade
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person taking the trade
    /// 1. `[writable]` The taker's token account for the token they send 
    /// 2. `[writable]` The taker's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The PDA's temp token account to get tokens from and eventually close
    /// 4. `[writable]` The initializer's main account to send their rent fees to
    /// 5. `[writable]` The initializer's token account that will receive tokens
    /// 6. `[writable]` The escrow account holding the escrow info
    /// 7. `[]` The token program
    /// 8. `[]` The PDA account
    Exchange {
        /// the amount the taker expects to be paid in the other token, as a u64 because that's the max possible supply of a token
        amount: u64,
    }
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  
13  
14  
15  
16  
17  
18  

The ix requires a total of 9 accounts. I will skip explaining them because you will see how they are used inside `processor.rs` and by now you should feel comfortable figuring this out by yourself!

Importantly, the ix also expects an amount. Why is this necessary? After all, Bob could simply look up the escrow information account in the explorer and check that in its state, it has a reference to a temporary token account with the amount of X tokens he is comfortable receiving for his Y tokens. Why should the amount he expects still be included in his ix? Try to figure it out yourself!

Show first hint

Show second hint

Show solution

We also need to adjust the match expression in the `unpack` function to include our new field.

    // inside unpack
    Ok(match tag {
        0 => Self::InitEscrow {
            amount: Self::unpack_amount(rest)?,
        },
        1 => Self::Exchange {
            amount: Self::unpack_amount(rest)?
        },
        _ => return Err(InvalidInstruction.into()),
    })
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  

And that's it for `instruction.rs`! At this point, you could try and finish the rest by yourself as an exercise but of course, you can also just go on with the guide. If you do decide to try it on your own first, make sure to look up the `invoke_signed` function, you will need it when transferring X tokens to Bob. Additionally, keep in mind you should somehow clean up the accounts created for the trade.

### [#](#processor-part-3-pdas-part-3) processor Part 3, PDAs Part 3

The `process` function inside `processor.rs` will not compile now because a match has to be _exhaustive_, i.e. match all the variants of the enum. Let's adjust it.

    match instruction {
        EscrowInstruction::InitEscrow { amount } => {
            msg!("Instruction: InitEscrow");
            Self::process_init_escrow(accounts, amount, program_id)
        },
        EscrowInstruction::Exchange { amount } => {
            msg!("Instruction: Exchange");
            Self::process_exchange(accounts, amount, program_id)
        }
    }
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  

Next, create the `process_exchange` function referenced here.

    // inside: impl Processor {}
    fn process_exchange(
        accounts: &[AccountInfo],
        amount_expected_by_taker: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let taker = next_account_info(account_info_iter)?;
    
        if !taker.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    
        let takers_sending_token_account = next_account_info(account_info_iter)?;
    
        let takers_token_to_receive_account = next_account_info(account_info_iter)?;
    
        let pdas_temp_token_account = next_account_info(account_info_iter)?;
        let pdas_temp_token_account_info =
            TokenAccount::unpack(&pdas_temp_token_account.try_borrow_data()?)?;
        let (pda, bump_seed) = Pubkey::find_program_address(&[b"escrow"], program_id);
    
        if amount_expected_by_taker != pdas_temp_token_account_info.amount {
            return Err(EscrowError::ExpectedAmountMismatch.into());
        }
    
        let initializers_main_account = next_account_info(account_info_iter)?;
        let initializers_token_to_receive_account = next_account_info(account_info_iter)?;
        let escrow_account = next_account_info(account_info_iter)?;
    
        let escrow_info = Escrow::unpack(&escrow_account.try_borrow_data()?)?;
    
        if escrow_info.temp_token_account_pubkey != *pdas_temp_token_account.key {
            return Err(ProgramError::InvalidAccountData);
        }
    
        if escrow_info.initializer_pubkey != *initializers_main_account.key {
            return Err(ProgramError::InvalidAccountData);
        }
    
        if escrow_info.initializer_token_to_receive_account_pubkey != *initializers_token_to_receive_account.key {
            return Err(ProgramError::InvalidAccountData);
        }
    
        let token_program = next_account_info(account_info_iter)?;
    
        let transfer_to_initializer_ix = spl_token::instruction::transfer(
            token_program.key,
            takers_sending_token_account.key,
            initializers_token_to_receive_account.key,
            taker.key,
            &[&taker.key],
            escrow_info.expected_amount,
        )?;
        msg!("Calling the token program to transfer tokens to the escrow's initializer...");
        invoke(
            &transfer_to_initializer_ix,
            &[
                takers_sending_token_account.clone(),
                initializers_token_to_receive_account.clone(),
                taker.clone(),
                token_program.clone(),
            ],
        )?;
        Ok(())
    }
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  
13  
14  
15  
16  
17  
18  
19  
20  
21  
22  
23  
24  
25  
26  
27  
28  
29  
30  
31  
32  
33  
34  
35  
36  
37  
38  
39  
40  
41  
42  
43  
44  
45  
46  
47  
48  
49  
50  
51  
52  
53  
54  
55  
56  
57  
58  
59  
60  
61  
62  
63  
64  
65  
66  

Up to this point, there's really nothing new. We get the accounts and do some checks on them, verifying that Bob has actually passed in the correct accounts with the correct values and that the amount in the PDA's X token account is what Bob expects. We then use _signature extension_ to make the token transfer to Alice's Y token account on Bob's behalf. You can fix the compilation errors yourself now (You'll have to import the spl\_token's account struct and [rename (opens new window)](https://doc.rust-lang.org/rust-by-example/mod/use.html) it to `TokenAccount` and add another error). Create the new error variant for the `ExpectedAmountMismatch` and pull the required modules into scope with `use`.

The last parts of the `process_exchange` function include something new again:

    ...
    let pda_account = next_account_info(account_info_iter)?;
    
    let transfer_to_taker_ix = spl_token::instruction::transfer(
        token_program.key,
        pdas_temp_token_account.key,
        takers_token_to_receive_account.key,
        &pda,
        &[&pda],
        pdas_temp_token_account_info.amount,
    )?;
    msg!("Calling the token program to transfer tokens to the taker...");
    invoke_signed(
        &transfer_to_taker_ix,
        &[
            pdas_temp_token_account.clone(),
            takers_token_to_receive_account.clone(),
            pda_account.clone(),
            token_program.clone(),
        ],
        &[&[&b"escrow"[..], &[bump_seed]]],
    )?;
    
    let close_pdas_temp_acc_ix = spl_token::instruction::close_account(
        token_program.key,
        pdas_temp_token_account.key,
        initializers_main_account.key,
        &pda,
        &[&pda]
    )?;
    msg!("Calling the token program to close pda's temp account...");
    invoke_signed(
        &close_pdas_temp_acc_ix,
        &[
            pdas_temp_token_account.clone(),
            initializers_main_account.clone(),
            pda_account.clone(),
            token_program.clone(),
        ],
        &[&[&b"escrow"[..], &[bump_seed]]],
    )?;
    
    Ok(())
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  
13  
14  
15  
16  
17  
18  
19  
20  
21  
22  
23  
24  
25  
26  
27  
28  
29  
30  
31  
32  
33  
34  
35  
36  
37  
38  
39  
40  
41  
42  
43  

Here we use the `invoke_signed` function to allow the PDA to sign something. Recall that a PDA is _bumped off_ the Ed25519 elliptic curve. Hence, there is no private key. The question is then, can PDAs sign CPIs? And the answer is

![](/assets/img/no_but_yes.f87dbd0e.jpg)

The PDA isn't actually signing the CPI in cryptographic fashion. In addition to the two arguments, the `invoke_signed` function takes a third one: the seeds that were used to create the PDA the CPI is supposed to be "signed" with. You might be surprised to find the bump seed there because you didn't define it as a seed. Well, the bump seed is the seed that the `find_program_address` function adds to make the address fall off the Ed25519 curve. Now,

> when a program calls `invoke_signed`, the runtime uses those seeds and the program id of the calling program to recreate the PDA and if it matches one of the given accounts inside `invoke_signed`'s arguments, that account's `signed` property will be set to true

No other program can fake this PDA because it is the runtime that sees which program is making the `invoke_signed` call. Only the escrow program will have the programId that will result in a PDA equal to one of the addresses in `invoke_signed`'s accounts argument.

We can see now that the first `invoke_signed` call transfers the tokens from the temp X token account to Bob's main X token account. The second one _closes_ the account. What does this mean? Recall that accounts are required to have a minimum balance to be rent exempt. Wouldn't it be nice if we could recover that balance when we no longer need an account? Turns out, we can and it's as simple as transferring the balance to a different account.

> If an account has no balance left, it will be purged from memory by the runtime after the transaction (you can see this when going navigating to an account that has been closed in the explorer)

Now we can see why we had to check whether the escrow state account was rent-exempt. If we had't and Alice were to pass in a non-rent-exempt account, the account balance might go to zero before Bob takes the trade. With the account gone, Alice would have no way to recover her tokens.

Since the temp token account is owned by the token program, only the token program may decrease the balance. And because this action requires permission of the authority of the token account (in this case the PDA), we use `invoke_signed` again.

To conclude this function and the program, we can do the same with the escrow state account.

    msg!("Closing the escrow account...");
    **initializers_main_account.lamports.borrow_mut() = initializers_main_account.lamports()
    .checked_add(escrow_account.lamports())
    .ok_or(EscrowError::AmountOverflow)?;
    **escrow_account.lamports.borrow_mut() = 0;
    *escrow_account.try_borrow_mut_data()? = &mut [];
    
    Ok(())
    

1  
2  
3  
4  
5  
6  
7  
8  

Transferring the lamports is as simple as adding the amount to one account and subtracting it from the other. We can adjust the balance of Alice's main account (she should get the lamports since she created the two accounts required for the trade) even though the escrow program is not the owner of her account because we are _crediting_ lamports to her account. Again, add the new error to `error.rs`.

Note that we are also setting the data field equal to an empty slice. Why is this necessary if the account is purged from memory after the transaction anyway? It is because this instruction is not necessarily the final instruction in the transaction. Thus, a subsequent transaction may read or even revive the data completely by making the account rent-exempt again. Depending on your program, forgetting to clear the data field can have dangerous consequences.

> In any call to a program that is of the "close" kind, i.e. where you set an account's lamports to zero so it's removed from memory after the transaction, make sure to either clear the data field or leave the data in a state that would be OK to be recovered by a subsequent transaction.

And that's it! That's the program. Well done!

![](/assets/img/applauding_sheep.d912d462.gif)

theory recap 📚

*   when a program calls `invoke_signed`, the runtime uses the given seeds and the program id of the calling program to recreate the PDA and if it matches one of the given accounts inside `invoke_signed`'s arguments, that account's `signed` property will be set to true
*   If an account has no balance left, it will be purged from memory by the runtime after the transaction (you can see this when going navigating to an account that has been closed in the explorer)
*   "closing" instructions must set the data field properly, even if the intent is to have the account be purged from memory after the transaction

### [#](#trying-out-the-program-understanding-bob-s-transaction-in-practice) Trying out the program, understanding Bob's transaction in practice

We can now try out the entire program. For this I've copied Alice's UI below so you don't have to scroll up and added another for Bob's side.

**Alice's UI**

**Bob's UI**

You'll need to build and deploy the updated program (This also means you cannot use the escrow account you created a while ago because it is owned by the outdated version of the program). Also, for a realistic test, create another account in sollet that acts as Bob. You can reuse the two token mint accounts you created a while ago. Create two new token accounts to hold Bob's X and Y tokens. Now you can create a new escrow as Alice and accept the trade as Bob.

#### [#](#understanding-what-just-happened) understanding what just happened

You can probably already tell what happened behind the scenes when you clicked "Take trade". The UI uses the escrow account pubkey to get the data from the escrow account, decodes it, and then uses the decoded data plus Bob's data to send the transaction. Here's the important code (without the decoding):

    const PDA = await PublicKey.findProgramAddress([Buffer.from("escrow")], programId);
    
    const exchangeInstruction = new TransactionInstruction({
        programId,
        data: Buffer.from(Uint8Array.of(1, ...new BN(takerExpectedXTokenAmount).toArray("le", 8))),
        keys: [
            { pubkey: takerAccount.publicKey, isSigner: true, isWritable: false },
            { pubkey: takerYTokenAccountPubkey, isSigner: false, isWritable: true },
            { pubkey: takerXTokenAccountPubkey, isSigner: false, isWritable: true },
            { pubkey: escrowState.XTokenTempAccountPubkey, isSigner: false, isWritable: true},
            { pubkey: escrowState.initializerAccountPubkey, isSigner: false, isWritable: true},
            { pubkey: escrowState.initializerYTokenAccount, isSigner: false, isWritable: true},
            { pubkey: escrowAccountPubkey, isSigner: false, isWritable: true },
            { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            { pubkey: PDA[0], isSigner: false, isWritable: false}
        ] 
    })
    
    await connection.sendTransaction(new Transaction().add(exchangeInstruction), [takerAccount], {skipPreflight: false, preflightCommitment: 'confirmed'});
    

1  
2  
3  
4  
5  
6  
7  
8  
9  
10  
11  
12  
13  
14  
15  
16  
17  
18  
19  

[#](#bonus-bugfixing) Bonus: Bugfixing!
---------------------------------------

There is a bug in this program. It's nothing critical. I've left it in because it does showcase the subtleties of programming on Solana. Can you find it?

Show first hint

Show second hint

Show solution

[#](#q-a) Q & A
---------------

This is a collection of questions that have been asked by readers whose answers would have made the guide itself too long. Feel free to [contact me (opens new window)](https://twitter.com/paulxpaulxpaulx) and ask away!

### [#](#is-there-really-a-need-for-a-temporary-account-for-alice-s-x-tokens) Is there really a need for a temporary account for Alice's X tokens?

**Q:** Is there really a need for a temporary account for Alice's X tokens?

*   a) Couldn't we just save the amount of X tokens Alice wants to trade inside the escrow state as well and then inside Bob's transaction have the escrow program make a CPI to the token program to deduct that amount from her account?

**A**: That wouldn't work because the `from` address of a token transfer needs to sign the transaction. Bob could ask Alice to sign his transaction but that would require more communication between Alice and Bob and thereby result in worse user experience. What you could use is a token account's (user space) `delegate` property but a token account can only have 1 delegate which means Alice could only have one ongoing escrow at a time.

What is possible, however, is to create the temporary account inside the init instruction and have it be directly owned by the escrow PDA instead of creating it in a previous instruction and then transferring the authority in the init instruction like it is done in this guide.

[#](#potential-improvements) Potential improvements
---------------------------------------------------

Here are some ideas to improve the user experience

*   build a better UI
    *   add a way to sign the tx without having to expose your private key (e.g. using [Solong (opens new window)](https://solongwallet.com/) or the [SOL Wallet Adapter (opens new window)](https://github.com/project-serum/sol-wallet-adapter))
    *   make it prettier
    *   add functionality to view an escrow's state given its address
*   add a `Cancel` endpoint to the program. Currently, Alice's tokens are stuck in limbo and she will not be able to recover them if Bob decides not to take the trade. Add an endpoint that allows Alice to cancel the ongoing escrow, transferring the X tokens back to her and closing the two created accounts. 🚨 If you implement cancel, you also need to add another check to prevent a frontrunning attack. Preventing it requires that Bob also sends the amount of Y tokens that he expects to send Alice (`expected_y_amount`) in addition to the amount of X tokens he expects from her. The check belongs in `process_exchange` and verifies that `escrow_info.expected_amount == expected_y_amount`. This prevents the following attack: Once Bob sends his Transaction and Alice sees it, Alice can cancel the escrow, reinitialise it at the same address but with a higher expected amount, thereby receiving more Y tokens than Bob expected to give her. Alternatively (or additionally), Bob could use a temporary token account himself so that in the case Alice frontruns him, his tx will fail because there's not enough Y tokens in his temporary token account. In addition, you also need to have Bob send his `expected_x_amount` (see the frontrunning section in [#instruction-rs-part-3-understanding-what-bob-s-transaction-should-do (opens new window)](https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/#instruction-rs-part-3-understanding-what-bob-s-transaction-should-do)). 🚨

[#](#further-reading) Further reading
-------------------------------------

Manual (De)serialization is a tedious and error-prone process. Check out the [borsh crate (opens new window)](https://crates.io/crates/borsh) which can automate this for you.

[Anchor (opens new window)](https://project-serum.github.io/anchor/getting-started/introduction.html), a framework for writing solana programs, goes further and simplifies the entire programming process.

*   [The docs (opens new window)](https://docs.solana.com)
*   [The solana-program rust docs (opens new window)](https://docs.rs/solana-program/latest/solana_program/index.html)
*   [The Solana medium account (opens new window)](https://medium.com/solana-labs)
*   [The token program (opens new window)](https://github.com/solana-labs/solana-program-library/tree/master/token/program)
*   [The token program docs (opens new window)](https://docs.rs/spl-token/latest/spl_token/)
*   [The system program (opens new window)](https://github.com/solana-labs/solana/blob/master/runtime/src/system_instruction_processor.rs)

[#](#edits-and-acknowledgements) Edits and acknowledgements
-----------------------------------------------------------

*   2021/01/18: added section on bugfixing
*   2021/01/31: renamed "nonce" to "bump seed"
*   2021/02/08: sollet now exports byte array instead of base58 encoded priv key
*   2021/03/19: updated token account link
*   2021/05/02: improve readability and explain how to get sollet byte array
*   2021/05/02: change "set\_token\_authority" to "set\_authority"
*   2021/05/11: add missing zeroing of data after process\_exchange
*   2021/05/19: update dependencies, add token program check warning
*   2021/05/29: sysvars can now be accessed without being passed in as an account
*   2021/08/11: added frontrunning quiz and improved flow
*   2021/08/30: added warning to potential improvements section regarding implementing cancel -- thanks to [Pierre Arowana (opens new window)](https://twitter.com/PierreArowana) and [William Arnold (opens new window)](https://twitter.com/windwardwill)
*   2021/09/27: improved cancel warning, added scripts, added borsh and anchor to further reading, and fixed broken link -- thanks to [Tony Ricciardi (opens new window)](https://twitter.com/TonyVRicciardi) for finding the link
*   2021/10/26: removed deprecated commitment levels (thanks to Sundeep Charan Ramkumar#2703 from discord), added AccountInfo helpers for data and lamports
*   2021/10/28: simplified token accounts diagrams
*   2021/11/19: added token analogy to clarify what X and Y are (thanks to @albttx from twitter)
*   2022/01/03: added alternative temporary account creation flow to Q&A section
*   2022/01/11: update versions and use "authority" instead of "user space owner"

* * *

[Intro & Motivation](#intro-motivation "Intro & Motivation")

[The Final Product](#the-final-product "The Final Product")

[What is an escrow?](#what-is-an-escrow "What is an escrow?")

[Building the escrow program - Alice's Transaction](#building-the-escrow-program-alice-s-transaction "Building the escrow program - Alice's Transaction")

[Setting up the project](#setting-up-the-project "Setting up the project")

[entrypoint.rs, programs, and accounts](#entrypoint-rs-programs-and-accounts "entrypoint.rs, programs, and accounts")

[instruction.rs Part 1, general code structure, and the beginning of the escrow program flow](#instruction-rs-part-1-general-code-structure-and-the-beginning-of-the-escrow-program-flow "instruction.rs Part 1, general code structure, and the beginning of the escrow program flow")

[The token program Part 1](#the-token-program-part-1 "The token program Part 1")

[Program Derived Addresses (PDAs) Part 1](#program-derived-addresses-pdas-part-1 "Program Derived Addresses (PDAs) Part 1")

[instruction.rs Part 2](#instruction-rs-part-2 "instruction.rs Part 2")

[error.rs](#error-rs "error.rs")

[processor.rs Part 1, Rent Part 1, starting to process the InitEscrow instruction](#processor-rs-part-1-rent-part-1-starting-to-process-the-initescrow-instruction "processor.rs Part 1, Rent Part 1, starting to process the InitEscrow instruction")

[state.rs](#state-rs "state.rs")

[Processor Part 2, PDAs Part 2, CPIs Part 1](#processor-part-2-pdas-part-2-cpis-part-1 "Processor Part 2, PDAs Part 2, CPIs Part 1")

[Trying out the program, understanding Alice's transaction](#trying-out-the-program-understanding-alice-s-transaction "Trying out the program, understanding Alice's transaction")

[Building the escrow program - Bob's Transaction](#building-the-escrow-program-bob-s-transaction "Building the escrow program - Bob's Transaction")

[instruction.rs Part 3, understanding what Bob's transaction should do](#instruction-rs-part-3-understanding-what-bob-s-transaction-should-do "instruction.rs Part 3, understanding what Bob's transaction should do")

[processor Part 3, PDAs Part 3](#processor-part-3-pdas-part-3 "processor Part 3, PDAs Part 3")

[Trying out the program, understanding Bob's transaction in practice](#trying-out-the-program-understanding-bob-s-transaction-in-practice "Trying out the program, understanding Bob's transaction in practice")

[Bonus: Bugfixing!](#bonus-bugfixing "Bonus: Bugfixing!")

[Q & A](#q-a "Q & A")

[Is there really a need for a temporary account for Alice's X tokens?](#is-there-really-a-need-for-a-temporary-account-for-alice-s-x-tokens "Is there really a need for a temporary account for Alice's X tokens?")

[Potential improvements](#potential-improvements "Potential improvements")

[Further reading](#further-reading "Further reading")

[Edits and acknowledgements](#edits-and-acknowledgements "Edits and acknowledgements")

*   [](https://github.com/paul-schaaf)
*   [](https://twitter.com/paulxpaulxpaulx)