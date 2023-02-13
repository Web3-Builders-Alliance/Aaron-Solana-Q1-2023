M]AMilz: i'm looking at an old anchor idl...when deployed it adds 
  "metadata": {
    "address": "7b8ZM4sAdsL9hP9yjQpYDYKoPK8jdgjnQtQJXUBku6Kf"
  }

to the end of idl.json but i'm not getting that on my build rn.
[11:06 AM]AMilz: weird
[11:06 AM]AMilz: i'll poke around
[11:08 AM]AMilz:
Your transaction signature 2FUdY4cHnqBU8gJTMe5byzb5WQW6Xru4WEd1jbh3qpGYwMr6SY4GTB6zVnRmJqzvhdDMcLgsYpgiaRrvvXTsx5oH
    ✔ Is initialized! (920ms)


  1 passing (922ms)

✨  Done in 2.68s.
qn-milano@aarons-mbp-2 deposit % 

very weird. i manually had to add the metadata field to my idl. and then use --skip-build to prevent the idl from being rebuilt