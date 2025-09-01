### Storage layout (LaunchToken) 

```text
_name string
_symbol string
_mediaURI string
unlocked bool
eventNonce uint256
totalFeeShare uint256
bondingShare mapping(address => uint256)

```

#### LaunchToken._beforeTokenTransfer(address,address,uint256) [INTERNAL]
```slithir
from_1(address) := phi(['from_1', 'TMP_2889', 'msg.sender', 'from_1', 'from_1'])
to_1(address) := phi(['to_1', 'to_1', 'TMP_2908', 'to_1', 'to_1'])
amount_1(uint256) := phi(['amount_1', 'amount_1', 'amount_1', 'amount_1', 'amount_1'])
launchpad_5(address) := phi(['launchpad_6', 'launchpad_0', 'launchpad_13', 'launchpad_1', 'launchpad_15', 'launchpad_4', 'launchpad_9'])
gteRouter_2(address) := phi(['gteRouter_0', 'gteRouter_1'])
unlocked_2(bool) := phi(['unlocked_0', 'unlocked_4', 'unlocked_1'])
 ! unlocked && from != launchpad && to != launchpad && to != gteRouter
TMP_2993 = UnaryType.BANG unlocked_2 
TMP_2994(bool) = from_1 != launchpad_5
TMP_2995(bool) = TMP_2993 && TMP_2994
TMP_2996(bool) = to_1 != launchpad_5
TMP_2997(bool) = TMP_2995 && TMP_2996
TMP_2998(bool) = to_1 != gteRouter_2
TMP_2999(bool) = TMP_2997 && TMP_2998
CONDITION TMP_2999
 revert TransfersDisabledWhileBonding()()
TMP_3000(None) = SOLIDITY_CALL revert TransfersDisabledWhileBonding()()
 ! unlocked
TMP_3001 = UnaryType.BANG unlocked_2 
CONDITION TMP_3001
 from != launchpad && to != launchpad && to != gteRouter
TMP_3002(bool) = from_1 != launchpad_5
TMP_3003(bool) = to_1 != launchpad_5
TMP_3004(bool) = TMP_3002 && TMP_3003
TMP_3005(bool) = to_1 != gteRouter_2
TMP_3006(bool) = TMP_3004 && TMP_3005
CONDITION TMP_3006
 revert TransfersDisabledWhileBonding()()
TMP_3007(None) = SOLIDITY_CALL revert TransfersDisabledWhileBonding()()
 from == launchpad && to != launchpad
TMP_3008(bool) = from_1 == launchpad_5
TMP_3009(bool) = to_1 != launchpad_5
TMP_3010(bool) = TMP_3008 && TMP_3009
CONDITION TMP_3010
 _increaseFeeShares(to,amount)
INTERNAL_CALL, LaunchToken._increaseFeeShares(address,uint256)(to_1,amount_1)
launchpad_6(address) := phi(['launchpad_9'])
 to != launchpad && to != gteRouter
TMP_3012(bool) = to_1 != launchpad_5
TMP_3013(bool) = to_1 != gteRouter_2
TMP_3014(bool) = TMP_3012 && TMP_3013
CONDITION TMP_3014
 revert TransfersDisabledWhileBonding()()
TMP_3015(None) = SOLIDITY_CALL revert TransfersDisabledWhileBonding()()
 from != launchpad
TMP_3016(bool) = from_1 != launchpad_5
CONDITION TMP_3016
 _decreaseFeeShares(from,amount)
INTERNAL_CALL, LaunchToken._decreaseFeeShares(address,uint256)(from_1,amount_1)
```
#### LaunchToken._decreaseFeeShares(address,uint256) [INTERNAL]
```slithir
account_1(address) := phi(['from_1'])
amount_1(uint256) := phi(['amount_1'])
launchpad_10(address) := phi(['launchpad_6', 'launchpad_0', 'launchpad_13', 'launchpad_1', 'launchpad_15', 'launchpad_4', 'launchpad_9'])
unlocked_3(bool) := phi(['unlocked_0', 'unlocked_4', 'unlocked_1'])
totalFeeShare_4(uint256) := phi(['totalFeeShare_0', 'totalFeeShare_3', 'totalFeeShare_6'])
bondingShare_4(mapping(address => uint256)) := phi(['bondingShare_6', 'bondingShare_0', 'bondingShare_3', 'bondingShare_4'])
 share = bondingShare[account]
REF_1075(uint256) -> bondingShare_4[account_1]
share_1(uint256) := REF_1075(uint256)
 share == 0 || account == address(0)
TMP_3027(bool) = share_1 == 0
TMP_3028 = CONVERT 0 to address
TMP_3029(bool) = account_1 == TMP_3028
TMP_3030(bool) = TMP_3027 || TMP_3029
CONDITION TMP_3030
 FeeShareDecreased(account,amount,_incEventNonce())
TMP_3031(uint256) = INTERNAL_CALL, LaunchToken._incEventNonce()()
Emit FeeShareDecreased(account_1,amount_4,TMP_3031)
 totalFeeShare -= amount
totalFeeShare_6(uint256) = totalFeeShare_5 - amount_4
 bondingShare[account] -= amount
REF_1076(uint256) -> bondingShare_5[account_1]
bondingShare_6(mapping(address => uint256)) := phi(['bondingShare_5'])
REF_1076(-> bondingShare_6) = REF_1076 - amount_4
 totalFeeShare == 0 && ! unlocked
TMP_3033(bool) = totalFeeShare_6 == 0
TMP_3034 = UnaryType.BANG unlocked_4 
TMP_3035(bool) = TMP_3033 && TMP_3034
CONDITION TMP_3035
 _endRewards()
INTERNAL_CALL, LaunchToken._endRewards()()
launchpad_12(address) := phi(['launchpad_15'])
 ILaunchpad(launchpad).decreaseStake(account,uint96(amount))
TMP_3037 = CONVERT launchpad_12 to ILaunchpad
TMP_3038 = CONVERT amount_4 to uint96
HIGH_LEVEL_CALL, dest:TMP_3037(ILaunchpad), function:decreaseStake, arguments:['account_1', 'TMP_3038']  
launchpad_13(address) := phi(['launchpad_6', 'launchpad_13', 'launchpad_12', 'launchpad_1', 'launchpad_15', 'launchpad_4', 'launchpad_9'])
 amount > share
TMP_3040(bool) = amount_1 > share_1
CONDITION TMP_3040
 amount = share
amount_2(uint256) := share_1(uint256)
 amount = amount
amount_3(uint256) := amount_1(uint256)
amount_4(uint256) := phi(['amount_2', 'amount_3'])
```
#### LaunchToken._endRewards() [INTERNAL]
```slithir
launchpad_14(address) := phi(['launchpad_6', 'launchpad_0', 'launchpad_13', 'launchpad_1', 'launchpad_15', 'launchpad_4', 'launchpad_9'])
 ILaunchpad(launchpad).endRewards()
TMP_3041 = CONVERT launchpad_14 to ILaunchpad
HIGH_LEVEL_CALL, dest:TMP_3041(ILaunchpad), function:endRewards, arguments:[]  
launchpad_15(address) := phi(['launchpad_6', 'launchpad_13', 'launchpad_14', 'launchpad_1', 'launchpad_15', 'launchpad_4', 'launchpad_9'])
 FeeShareConcluded(block.timestamp,_incEventNonce())
TMP_3043(uint256) = INTERNAL_CALL, LaunchToken._incEventNonce()()
Emit FeeShareConcluded(block.timestamp,TMP_3043)
```
#### LaunchToken._incEventNonce() [INTERNAL]
```slithir
eventNonce_1(uint256) := phi(['eventNonce_2', 'eventNonce_0'])
 nonce = eventNonce
nonce_1(uint256) := eventNonce_1(uint256)
 eventNonce ++
TMP_3045(uint256) := eventNonce_1(uint256)
eventNonce_2(uint256) = eventNonce_1 (c)+ 1
 nonce
RETURN nonce_1
```
#### LaunchToken._increaseFeeShares(address,uint256) [INTERNAL]
```slithir
account_1(address) := phi(['to_1'])
amount_1(uint256) := phi(['amount_1'])
launchpad_7(address) := phi(['launchpad_6', 'launchpad_0', 'launchpad_13', 'launchpad_1', 'launchpad_15', 'launchpad_4', 'launchpad_9'])
totalFeeShare_1(uint256) := phi(['totalFeeShare_0', 'totalFeeShare_3', 'totalFeeShare_6'])
bondingShare_1(mapping(address => uint256)) := phi(['bondingShare_6', 'bondingShare_0', 'bondingShare_3', 'bondingShare_4'])
 amount == 0 || account == address(0)
TMP_3018(bool) = amount_1 == 0
TMP_3019 = CONVERT 0 to address
TMP_3020(bool) = account_1 == TMP_3019
TMP_3021(bool) = TMP_3018 || TMP_3020
CONDITION TMP_3021
 FeeShareIncreased(account,amount,_incEventNonce())
TMP_3022(uint256) = INTERNAL_CALL, LaunchToken._incEventNonce()()
Emit FeeShareIncreased(account_1,amount_1,TMP_3022)
 totalFeeShare += amount
totalFeeShare_3(uint256) = totalFeeShare_2 + amount_1
 bondingShare[account] += amount
REF_1073(uint256) -> bondingShare_2[account_1]
bondingShare_3(mapping(address => uint256)) := phi(['bondingShare_2'])
REF_1073(-> bondingShare_3) = REF_1073 + amount_1
 ILaunchpad(launchpad).increaseStake(account,uint96(amount))
TMP_3024 = CONVERT launchpad_8 to ILaunchpad
TMP_3025 = CONVERT amount_1 to uint96
HIGH_LEVEL_CALL, dest:TMP_3024(ILaunchpad), function:increaseStake, arguments:['account_1', 'TMP_3025']  
launchpad_9(address) := phi(['launchpad_6', 'launchpad_13', 'launchpad_1', 'launchpad_15', 'launchpad_8', 'launchpad_4', 'launchpad_9'])
```
#### LaunchToken.constructor(string,string,string,address) [PUBLIC]
```slithir
 _name = name_
_name_1(string) := name__1(string)
 _symbol = symbol_
_symbol_1(string) := symbol__1(string)
 _mediaURI = mediaUri_
_mediaURI_1(string) := mediaUri__1(string)
 gteRouter = gteRouter_
gteRouter_1(address) := gteRouter__1(address)
 launchpad = msg.sender
launchpad_1(address) := msg.sender(address)
```
#### LaunchToken.mediaURI() [PUBLIC]
```slithir
_mediaURI_2(string) := phi(['_mediaURI_1', '_mediaURI_0'])
 _mediaURI
RETURN _mediaURI_2
```
#### LaunchToken.mint(uint256) [EXTERNAL]
```slithir
launchpad_2(address) := phi(['launchpad_6', 'launchpad_0', 'launchpad_13', 'launchpad_1', 'launchpad_15', 'launchpad_4', 'launchpad_9'])
 _mint(launchpad,amount)
INTERNAL_CALL, ERC20._mint(address,uint256)(launchpad_3,amount_1)
 totalSupply() > type()(uint96).max
TMP_2987(uint256) = INTERNAL_CALL, ERC20.totalSupply()()
TMP_2989(uint96) := 79228162514264337593543950335(uint96)
TMP_2990(bool) = TMP_2987 > TMP_2989
CONDITION TMP_2990
 revert TotalSupplyExceedsMaxShares()()
TMP_2991(None) = SOLIDITY_CALL revert TotalSupplyExceedsMaxShares()()
 onlyLaunchpad()
MODIFIER_CALL, LaunchToken.onlyLaunchpad()()
```
#### LaunchToken.name() [PUBLIC]
```slithir
_name_2(string) := phi(['_name_0', '_name_1'])
 _name
RETURN _name_2
```
#### Distributor.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 _OWNER_SLOT = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927
 _ROLE_0 = 1 << 0
 _ROLE_1 = 1 << 1
 _ROLE_2 = 1 << 2
 _ROLE_3 = 1 << 3
 _ROLE_4 = 1 << 4
 _ROLE_5 = 1 << 5
 _ROLE_6 = 1 << 6
 _ROLE_7 = 1 << 7
 _ROLE_8 = 1 << 8
 _ROLE_9 = 1 << 9
 _ROLE_10 = 1 << 10
 _ROLE_11 = 1 << 11
 _ROLE_12 = 1 << 12
 _ROLE_13 = 1 << 13
 _ROLE_14 = 1 << 14
 _ROLE_15 = 1 << 15
 _ROLE_16 = 1 << 16
 _ROLE_17 = 1 << 17
 _ROLE_18 = 1 << 18
 _ROLE_19 = 1 << 19
 _ROLE_20 = 1 << 20
 _ROLE_21 = 1 << 21
 _ROLE_22 = 1 << 22
 _ROLE_23 = 1 << 23
 _ROLE_24 = 1 << 24
 _ROLE_25 = 1 << 25
 _ROLE_26 = 1 << 26
 _ROLE_27 = 1 << 27
 _ROLE_28 = 1 << 28
 _ROLE_29 = 1 << 29
 _ROLE_30 = 1 << 30
 _ROLE_31 = 1 << 31
 _ROLE_32 = 1 << 32
 _ROLE_33 = 1 << 33
 _ROLE_34 = 1 << 34
 _ROLE_35 = 1 << 35
 _ROLE_36 = 1 << 36
 _ROLE_37 = 1 << 37
 _ROLE_38 = 1 << 38
 _ROLE_39 = 1 << 39
 _ROLE_40 = 1 << 40
 _ROLE_41 = 1 << 41
 _ROLE_42 = 1 << 42
 _ROLE_43 = 1 << 43
 _ROLE_44 = 1 << 44
 _ROLE_45 = 1 << 45
 _ROLE_46 = 1 << 46
 _ROLE_47 = 1 << 47
 _ROLE_48 = 1 << 48
 _ROLE_49 = 1 << 49
 _ROLE_50 = 1 << 50
 _ROLE_51 = 1 << 51
 _ROLE_52 = 1 << 52
 _ROLE_53 = 1 << 53
 _ROLE_54 = 1 << 54
 _ROLE_55 = 1 << 55
 _ROLE_56 = 1 << 56
 _ROLE_57 = 1 << 57
 _ROLE_58 = 1 << 58
 _ROLE_59 = 1 << 59
 _ROLE_60 = 1 << 60
 _ROLE_61 = 1 << 61
 _ROLE_62 = 1 << 62
 _ROLE_63 = 1 << 63
 _ROLE_64 = 1 << 64
 _ROLE_65 = 1 << 65
 _ROLE_66 = 1 << 66
 _ROLE_67 = 1 << 67
 _ROLE_68 = 1 << 68
 _ROLE_69 = 1 << 69
 _ROLE_70 = 1 << 70
 _ROLE_71 = 1 << 71
 _ROLE_72 = 1 << 72
 _ROLE_73 = 1 << 73
 _ROLE_74 = 1 << 74
 _ROLE_75 = 1 << 75
 _ROLE_76 = 1 << 76
 _ROLE_77 = 1 << 77
 _ROLE_78 = 1 << 78
 _ROLE_79 = 1 << 79
 _ROLE_80 = 1 << 80
 _ROLE_81 = 1 << 81
 _ROLE_82 = 1 << 82
 _ROLE_83 = 1 << 83
 _ROLE_84 = 1 << 84
 _ROLE_85 = 1 << 85
 _ROLE_86 = 1 << 86
 _ROLE_87 = 1 << 87
 _ROLE_88 = 1 << 88
 _ROLE_89 = 1 << 89
 _ROLE_90 = 1 << 90
 _ROLE_91 = 1 << 91
 _ROLE_92 = 1 << 92
 _ROLE_93 = 1 << 93
 _ROLE_94 = 1 << 94
 _ROLE_95 = 1 << 95
 _ROLE_96 = 1 << 96
 _ROLE_97 = 1 << 97
 _ROLE_98 = 1 << 98
 _ROLE_99 = 1 << 99
 _ROLE_100 = 1 << 100
 _ROLE_101 = 1 << 101
 _ROLE_102 = 1 << 102
 _ROLE_103 = 1 << 103
 _ROLE_104 = 1 << 104
 _ROLE_105 = 1 << 105
 _ROLE_106 = 1 << 106
 _ROLE_107 = 1 << 107
 _ROLE_108 = 1 << 108
 _ROLE_109 = 1 << 109
 _ROLE_110 = 1 << 110
 _ROLE_111 = 1 << 111
 _ROLE_112 = 1 << 112
 _ROLE_113 = 1 << 113
 _ROLE_114 = 1 << 114
 _ROLE_115 = 1 << 115
 _ROLE_116 = 1 << 116
 _ROLE_117 = 1 << 117
 _ROLE_118 = 1 << 118
 _ROLE_119 = 1 << 119
 _ROLE_120 = 1 << 120
 _ROLE_121 = 1 << 121
 _ROLE_122 = 1 << 122
 _ROLE_123 = 1 << 123
 _ROLE_124 = 1 << 124
 _ROLE_125 = 1 << 125
 _ROLE_126 = 1 << 126
 _ROLE_127 = 1 << 127
 _ROLE_128 = 1 << 128
 _ROLE_129 = 1 << 129
 _ROLE_130 = 1 << 130
 _ROLE_131 = 1 << 131
 _ROLE_132 = 1 << 132
 _ROLE_133 = 1 << 133
 _ROLE_134 = 1 << 134
 _ROLE_135 = 1 << 135
 _ROLE_136 = 1 << 136
 _ROLE_137 = 1 << 137
 _ROLE_138 = 1 << 138
 _ROLE_139 = 1 << 139
 _ROLE_140 = 1 << 140
 _ROLE_141 = 1 << 141
 _ROLE_142 = 1 << 142
 _ROLE_143 = 1 << 143
 _ROLE_144 = 1 << 144
 _ROLE_145 = 1 << 145
 _ROLE_146 = 1 << 146
 _ROLE_147 = 1 << 147
 _ROLE_148 = 1 << 148
 _ROLE_149 = 1 << 149
 _ROLE_150 = 1 << 150
 _ROLE_151 = 1 << 151
 _ROLE_152 = 1 << 152
 _ROLE_153 = 1 << 153
 _ROLE_154 = 1 << 154
 _ROLE_155 = 1 << 155
 _ROLE_156 = 1 << 156
 _ROLE_157 = 1 << 157
 _ROLE_158 = 1 << 158
 _ROLE_159 = 1 << 159
 _ROLE_160 = 1 << 160
 _ROLE_161 = 1 << 161
 _ROLE_162 = 1 << 162
 _ROLE_163 = 1 << 163
 _ROLE_164 = 1 << 164
 _ROLE_165 = 1 << 165
 _ROLE_166 = 1 << 166
 _ROLE_167 = 1 << 167
 _ROLE_168 = 1 << 168
 _ROLE_169 = 1 << 169
 _ROLE_170 = 1 << 170
 _ROLE_171 = 1 << 171
 _ROLE_172 = 1 << 172
 _ROLE_173 = 1 << 173
 _ROLE_174 = 1 << 174
 _ROLE_175 = 1 << 175
 _ROLE_176 = 1 << 176
 _ROLE_177 = 1 << 177
 _ROLE_178 = 1 << 178
 _ROLE_179 = 1 << 179
 _ROLE_180 = 1 << 180
 _ROLE_181 = 1 << 181
 _ROLE_182 = 1 << 182
 _ROLE_183 = 1 << 183
 _ROLE_184 = 1 << 184
 _ROLE_185 = 1 << 185
 _ROLE_186 = 1 << 186
 _ROLE_187 = 1 << 187
 _ROLE_188 = 1 << 188
 _ROLE_189 = 1 << 189
 _ROLE_190 = 1 << 190
 _ROLE_191 = 1 << 191
 _ROLE_192 = 1 << 192
 _ROLE_193 = 1 << 193
 _ROLE_194 = 1 << 194
 _ROLE_195 = 1 << 195
 _ROLE_196 = 1 << 196
 _ROLE_197 = 1 << 197
 _ROLE_198 = 1 << 198
 _ROLE_199 = 1 << 199
 _ROLE_200 = 1 << 200
 _ROLE_201 = 1 << 201
 _ROLE_202 = 1 << 202
 _ROLE_203 = 1 << 203
 _ROLE_204 = 1 << 204
 _ROLE_205 = 1 << 205
 _ROLE_206 = 1 << 206
 _ROLE_207 = 1 << 207
 _ROLE_208 = 1 << 208
 _ROLE_209 = 1 << 209
 _ROLE_210 = 1 << 210
 _ROLE_211 = 1 << 211
 _ROLE_212 = 1 << 212
 _ROLE_213 = 1 << 213
 _ROLE_214 = 1 << 214
 _ROLE_215 = 1 << 215
 _ROLE_216 = 1 << 216
 _ROLE_217 = 1 << 217
 _ROLE_218 = 1 << 218
 _ROLE_219 = 1 << 219
 _ROLE_220 = 1 << 220
 _ROLE_221 = 1 << 221
 _ROLE_222 = 1 << 222
 _ROLE_223 = 1 << 223
 _ROLE_224 = 1 << 224
 _ROLE_225 = 1 << 225
 _ROLE_226 = 1 << 226
 _ROLE_227 = 1 << 227
 _ROLE_228 = 1 << 228
 _ROLE_229 = 1 << 229
 _ROLE_230 = 1 << 230
 _ROLE_231 = 1 << 231
 _ROLE_232 = 1 << 232
 _ROLE_233 = 1 << 233
 _ROLE_234 = 1 << 234
 _ROLE_235 = 1 << 235
 _ROLE_236 = 1 << 236
 _ROLE_237 = 1 << 237
 _ROLE_238 = 1 << 238
 _ROLE_239 = 1 << 239
 _ROLE_240 = 1 << 240
 _ROLE_241 = 1 << 241
 _ROLE_242 = 1 << 242
 _ROLE_243 = 1 << 243
 _ROLE_244 = 1 << 244
 _ROLE_245 = 1 << 245
 _ROLE_246 = 1 << 246
 _ROLE_247 = 1 << 247
 _ROLE_248 = 1 << 248
 _ROLE_249 = 1 << 249
 _ROLE_250 = 1 << 250
 _ROLE_251 = 1 << 251
 _ROLE_252 = 1 << 252
 _ROLE_253 = 1 << 253
 _ROLE_254 = 1 << 254
 _ROLE_255 = 1 << 255
 ADMIN_ROLE = _ROLE_0
 _checkRoles(roles)
INTERNAL_CALL, OwnableRoles._checkRoles(uint256)(roles_1)
roles_1(uint256) := phi(['ADMIN_ROLE_1'])
 _checkOwnerOrRoles(roles)
INTERNAL_CALL, OwnableRoles._checkOwnerOrRoles(uint256)(roles_1)
 _checkRolesOrOwner(roles)
INTERNAL_CALL, OwnableRoles._checkRolesOrOwner(uint256)(roles_1)
 _checkOwner()
INTERNAL_CALL, Ownable._checkOwner()()
launchpad_2(address) := phi(['launchpad_0', 'launchpad_1'])
 msg.sender != launchpad
TMP_2649(bool) = msg.sender != launchpad_2
CONDITION TMP_2649
 revert Unauthorized()()
TMP_2650(None) = SOLIDITY_CALL revert Unauthorized()()
```
#### LaunchToken.symbol() [PUBLIC]
```slithir
_symbol_2(string) := phi(['_symbol_0', '_symbol_1'])
 _symbol
RETURN _symbol_2
```
#### LaunchToken.unlock() [EXTERNAL]
```slithir
 unlocked = true
unlocked_1(bool) := True(bool)
 TransfersUnlocked(block.timestamp,_incEventNonce())
TMP_2983(uint256) = INTERNAL_CALL, LaunchToken._incEventNonce()()
Emit TransfersUnlocked(block.timestamp,TMP_2983)
 onlyLaunchpad()
MODIFIER_CALL, LaunchToken.onlyLaunchpad()()
```
#### ILaunchpad.decreaseStake(address,uint96) [EXTERNAL]
```slithir

```
#### ILaunchpad.endRewards() [EXTERNAL]
```slithir

```
#### ILaunchpad.increaseStake(address,uint96) [EXTERNAL]
```slithir

```
