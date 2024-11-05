/*
    This file is part of Donet.

    Copyright © 2024 Max Rodriguez <me@maxrdz.com>

    Donet is free software; you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License,
    as published by the Free Software Foundation, either version 3
    of the License, or (at your option) any later version.

    Donet is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public
    License along with Donet. If not, see <https://www.gnu.org/licenses/>.
*/

//! This test is an integration test for the DC language
//! as defined in the donet-core library.

use donet_core::dcfile::DCFile;
use donet_core::dconfig::*;
use donet_core::read_dc;

fn main() {
    // Use default DC language configuration options.
    //
    // DC_MULTIPLE_INHERITANCE == true
    // DC_SORT_INHERITANCE_BY_FILE == true
    // DC_VIRTUAL_INHERITANCE == true
    let conf: DCFileConfig = DCFileConfig::default();

    let dc: &str = "

// This is a C++ Style comment.

/*
    This is a C style comment.
*/

from example_views import DistributedDonut
from views import DistributedDonut/AI/OV
from views/AI/OV/UD import DistributedDonut/AI/OV/UD
from views/AI import DistributedDonut
from game.views.Donut/AI import DistributedDonut/AI
from views import *
from db.char import DistributedDonut

typedef uint8 bool; // deprecated, but handled gracefully

keyword p2p;
keyword monkey;
keyword unreliable;
keyword db;

struct GiftItem {
    blob Item;
    string giftTag;
};

struct Activity {
    string activityName;
    uint8 activityId;
};

struct Party {
    activity activities[];
    uint8 status;
};

struct Fixture {
    bool;
    int32/10 x;
    int32/10 y;
    int32/10 z;
    int16/10 h;
    int16/10 p;
    int16/10 r;
    string state;
};

struct MethodDataTypesTest {
    Char character;
    blob Item;
    blob32 pandaOnlyToken;
    float32 astronOnlyToken;
    string giftTag;
    int32(0-990999) testMethodValue;
    int8(-1-1) testNegativeValues;
    int8(-5--99) testNegativeValuesPartTwo;
    int8(+0-+9) plusForPositiveForSomeReason;
    int8array arrayDataTypeTest;
    int16array anotherArray;
    int32array evenMoreComplexArray;
    uint8array byteArray;
    uint16array unsignedIntegerArray;
    uint32array unsignedLongArray;
    uint32uint8array thisWeirdPandaArrayType;
};

struct TransformedTypesTest {
    int32%360 angle;
    int32%360/1000 floatingPointAngle;
    int32/1000 efficientFloatIn32Bits;
    float32 waitIsntAstronsFloat32TheSame;
    int16(int32) forTheStaticallyTypedLanguages;
    int16(float64)(0.0-1.0) withRangeTest;
    int16(float32)%360/10.0 anotherTest;
    int16(uint32)/10 moreTests;
    bool thisIsLiterallyJustAn8BitInt;
    uint16/1000(0-1) youCanStackThemToo;
    int64/10000(+50-+999) [] thisIsValid;
    int8%10(0-10) anotherOne;
    int32('a'-'b') numericRangeWithChar;
    float32(0.1-0.99) floatingRange;
    float32%10.0 modulusWithFloat;
    float32(float64)%10.0 coverage;
    int16%100/10(-80-+100) lastTest;
};

struct NumericRanges {
    int8(0-1) thisIsLiterallyABoolean;
    int64(-5) signedRange;
    int64(+50-+999) thisIsValid;
    int32('a') numericRangeWithChar;
    int32('a'-'z') rangeMinMaxWithChar;
    float32(0.1-0.99) floatingRange;
    float32(0.1) anotherFloatRange;
    int32() pandaSaysThisIsLegal;
};

struct ParamsWithDefaultTest {
    string = \"\";
    MyStruct[] = [];
    MyStruct strukt[] = [];
    int32 = -99;
    string = \"VALUE\";
    string = 0xabcdef;
    uint16 accessLevel = 0;
    bool = false;
};

struct ArrayExpansionsTest {
    uint8array test = [0];
    uint8array test2 = [0 * 10];
    int8array test3 = [-1 * 10];
    int8array test4 = [5 * 5, 10 * 10, -2 * 4];
    uint8array test5 = [0xf * 10];
    uint8array test6 = [\"TEST\" * 2];
};

struct ArrayRangesTest {
    uint8 test['a'];
    uint8 test2[9];
    uint32uint8array[0-1] test3;
    uint32uint8array[0-1][9-99] test4;
    uint8 test5['a'-'b'] [ ];
    string(5) test6; // builtin array type
};

struct BuffData {
    switch (uint16) {
        case 0:
            break;
        case 1:
            uint8 val1;
            break;
        case 2:
            uint8 val1;
            uint8 val2;
            break;
        case 3:
            uint8 val1;
            break;
        case 4:
            int16/100 val1;
            break;
    };
    switch OptionalName (uint8) {
        case 0:
            break;
        default:
            uint8 value[0-5];
            uint32uint8array value2;
            SomeStruct value3;
            break;
    };
    switch WithDefault (char) {
        case 'a':
            break;
        case 'b':
        case 'c':
        case 'd':
        default:
            string val1;
            break;
    };
};

dclass DeveloperDefinedKeywords {
    testingField() p2p;
};

dclass Avatar {
    string name;
    uint16 health;

    set_xyzh(int16 x, int16 y, int16 z, int16 h) broadcast required;
    indicate_intent(int16 / 10, int16 / 10) ownsend airecv;
};

dclass OfflineShardManager : DistributedObject {
    clientSetZone(uint32) airecv clsend;
    requestZoneIdMessage(uint32, uint16) airecv clsend;
    requestZoneIdResponse(uint32, uint16);
};

dclass ShardStats {
    setShardId(uint32) broadcast required ram;
    setAvatarCount(uint32) broadcast required ram;
    setNewAvatarCount(uint32) broadcast required ram;
    setStats : setAvatarCount, setNewAvatarCount;
};

dclass DistributedChild : Parent, Parent2 {
};

dclass AtomicFields {
    simple();
    keyw0rd() ram;
    keywords() db ownsend airecv;
    parameter(string);
    params(bool, char, float64);
    named_params(bool flag = true, string text);
};

dclass MolecularFields {
    setXYZ : setX, setY, setZ;
    setPos : setXYZ;
    setXY : setX, setY;
    setHPR : setH, setP, setR;
};

    ";

    let _: DCFile<'_> = read_dc(conf, dc.into()).unwrap();
}
