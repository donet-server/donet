// DONET SOFTWARE
// Copyright (c) 2023, Donet Authors.

// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License version 3.
// You should have received a copy of this license along
// with this source code in a file named "LICENSE."
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program; if not, write to the Free Software Foundation,
// Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

#[allow(dead_code)] // FIXME: Remove once project matures
mod messages {

    enum Client { 
        Hello =                         1,
        HelloResp =                     2,
        // Sent by the client when it's leaving.
        Disconnect =                    3,
        // Sent by the server when it decides to force drop the client.
        Eject =                         4,
        Heartbeat =                     5,
        
        ObjectSetField =                120,
        ObjectSetFields =               121,
        ObjectLeaving =                 132,
        ObjectLeavingOwner =            161,
        EnterObjectRequired =           142,
        EnterObjectRequiredOther =      143,
        EnterObjectRequiredOwner =      172,
        EnterObjectRequiredOwnerOther = 173,

        DoneInterestResp =              204,

        AddInterest =                   200,
        AddInterestMultiple =           201,
        RemoveInterest =                203,
        ObjectLocation =                140,
    }

    // ---------- Internal Messages ---------- //
    
    enum ClientAgent {
        SetState =                      1000,
        SetClientID =                   1001,
        SendDatagram =                  1002,
        Eject =                         1004,
        Drop =                          1005,
        GetNetworkAddress =             1006,
        GetNetworkAddressResp =         1007,
        DeclareObject =                 1010,
        UndeclareObject =               1011,
        AddSessionObject =              1012,
        RemoveSessionObject =           1013,
        SetFieldsSendable =             1014,
        OpenChannel =                   1100,
        CloseChannel =                  1101,
        AddPostRemove =                 1110,
        ClearPostRemoves =              1111,
        AddInterest =                   1200,
        AddInterestMultiple =           1201,
        RemoveInterest =                1203,
    }

    enum StateServer {
        CreateObjectWithRequired =              2000,
        CreateObjectWithRequiredOther =         2001,
        DeleteAIObjects =                       2009,
        ObjectGetField =                        2010,
        ObjectGetFieldResp =                    2011,
        ObjectGetFields =                       2012,
        ObjectGetFieldsResp =                   2013,
        ObjectGetAll =                          2014,
        ObjectGetAllResp =                      2015,
        ObjectSetField =                        2020,
        ObjectSetFields =                       2021,
        ObjectDeleteFieldRAM =                  2030,
        ObjectDeleteFieldsRAM =                 2031,
        ObjectDeleteRAM =                       2032,
        ObjectSetLocation =                     2040,
        ObjectChangingLocation =                2041,
        ObjectEnterLocationWithRequired =       2042,
        ObjectEnterLocationWithRequiredOther =  2043,
        ObjectGetLocation =                     2044,
        ObjectGetLocationResp =                 2045,
        ObjectSetAI =                           2050,
        ObjectChangingAI =                      2051,
        ObjectEnterAIWithRequired =             2052,
        ObjectEnterAIWithRequiredOther =        2053,
        ObjectGetAI =                           2054,
        ObjectGetAIResp =                       2055,
        ObjectSetOwner =                        2060,
        ObjectChangingOwner =                   2061,
        ObjectEnterOwnerWithRequired =          2062,
        ObjectEnterOwnerWithRequiredOther =     2063,
        ObjectGetOwner =                        2064,
        ObjectGetOwnerResp =                    2065,
        ObjectGetZoneObjects =                  2100,
        ObjectGetZonesObjects =                 2102,
        ObjectGetChildren =                     2104,
        ObjectGetZoneCount =                    2110,
        ObjectGetZoneCountResp =                2111,
        ObjectGetZonesCount =                   2112,
        ObjectGetZonesCountResp =               2113,
        ObjectGetChildCount =                   2114,
        ObjectGetChildCountResp =               2115,
        ObjectDeleteZone =                      2120,
        ObjectDeleteZones =                     2122,
        ObjectDeleteChildren =                  2124,
    }

    enum DBSS {
        ObjectActivateWithDefaults =        2200,
        ObjectActivateWithDefaultsOther =   2201,
        ObjectGetActivated =                2207,
        ObjectGetActivatedResp =            2208,
        ObjectDeleteFieldDisk =             2230,
        ObjectDeleteFieldsDisk =            2231,
        ObjectDeleteDisk =                  2232,
    }

    enum DBServer {
        CreateObject =                  3000,
        CreateObjectResp =              3001,
        ObjectGetField =                3010,
        ObjectGetFieldResp =            3011,
        ObjectGetFields =               3012,
        ObjectGetFieldsResp =           3013,
        ObjectGetAll =                  3014,
        ObjectGetAllResp =              3015,
        ObjectSetField =                3020,
        ObjectSetFields =               3021,
        ObjectSetFieldIfEquals =        3022,
        ObjectSetFieldIfEqualsResp =    3023,
        ObjectSetFieldsIfEquals =       3024,
        ObjectSetFieldsIfEqualsResp =   3025,
        ObjectSetFieldIfEmpty =         3026,
        ObjectSetFieldIfEmptyResp =     3027,
        ObjectDeleteField =             3030,
        ObjectDeleteFields =            3031,
        ObjectDelete =                  3032,
    }

    enum Control {
        AddChannel =                    9000,
        RemoveChannel =                 9001,
        AddRange =                      9002,
        RemoveRange =                   9003,
        AddPostRemove =                 9010,
        ClearPostRemoves =              9011,
    }
}
