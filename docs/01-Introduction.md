<img src="logo/donet_banner.png" align="right" width="40%"/>

# 01 - Introduction to DoNet

DoNet is a free and open-source server software, designed for powering massive
multiplayer online games. The architecture of DoNet is focused on four main
objectives: Network **culling**, short-term & long-term data **persistence**, 
**security**, and **scalability**.

The architecture of this project is inspired by the OTP (Online Theme Park) server, 
which was developed by Disney Interactive (formerly known as Disney VR Studios) 
and used from 2001 to 2013 to power massive multiplayer online games such as 
Toontown Online, Pirates of the Caribbean Online, and others. DoNet is licensed 
under the GNU Affero General Public License (AGPLv3).

DoNet is designed to distribute the workload of operating a virtual world (or online 
application) by separating it's fundamental functions into different modules. In a 
production environment, many instances of DoNet can be running in different machines, 
each serving a specific role in the cluster while communicating with each other over 
the DoNet protocol.

## Overview

Distributed Networking is the high-level network API of the Panda3D engine. When a 
distributed object is created, all interested clients will automatically create a 
copy of that object. Updates to the object will automatically propagate to the copies.

The distributed network is composed of several layers: The DC file (*.dc), which defines 
the communication, ServerRepositories which handle communication between clients, 
ClientRepositories which interact and manage the Distributed Objects, 
and the Distributed Objects themselves.

The architecture of a DoNet server cluster is made up of 6 different units, or modules:

### **[CA] - Client Agent**

  The Client Agent component manages connections with **anonymous clients** that are connecting 
  from outside of the internal server network. Clients do not directly communicate with the 
  DoNet cluster. Instead, the Client Agent relays client messages over to the network. This 
  component provides two of the main features in a DoNet server cluster, which is **security** 
  and **network culling**. It reads the DC file(s) given to it and approves all messages from 
  clients that conform to the communication 'contract' defined in the DC file. It also checks 
  for other details, such as the clients' ownership over Distributed Objects and the visibility 
  (or location) of Distributed Objects. The Client Agent acts as the border between the untrusted 
  clients and the internal server network's 'safe zone'.
  
### **[MD] - Message Director**
  
  The Message Director listens for messages from other components in a DoNet server cluster, 
  and **routes** them to other components based on the recipients in the messages received. A message 
  is a blob of binary data sent over the network, with a maximum size of approximately **64 kilobytes**. 
  The routing is performed by means of routing identifiers called **channels**, where a message contains any 
  number of destination channels, and most messages include a source, or sender channel. Each component tells 
  the Message Director which channels it would like to **subscribe** to, and receives messages sent to its 
  subscribed channels.
  
### **[SS] - State Server**
  
  The State Server

### **[DB] - Database Server**
  
  The Database Server
  
### **[DBSS] - Database State Server**
  
  The Database State Server (DBSS for short) is a kind of hybrid component of a State 
  Server and a Database Server.
  
### **[EL] - Event Logger**
  
  The Event Logger listens to the Message Director for log messages that it should
  write to the disk. These log messages can be sent from AI processes, which are sent to
  a Message Director instance, which then routes it to the Event Logger.

DoNet can be configured to serve as all these roles under one process, which is 
handy for development on your local machine. For a production environment, many instances
of DoNet can be running on different machines and configured to serve as one component each. 
This configuration would be in a .par file that the DoNet process would read on startup.

There are many acronyms that will be used in the documentation. Please review the list 
below to learn and understand the different concepts and terms used in DoNet.

- **DO**

  Distributed Object. Represents an object in the virtual world.
  
- **DOG**
  
  Distributed Object Global. Similar to a Distributed Object, but is known globally
  and always accessible by all participants. (clients and AI processes)
  
- **DoId**
  
  Distributed Object Identifier. This is a 32-bit long identifier that is generated
  at runtime to identify a Distributed Object that exists in the State Server.
  
- **AI**
  
  Artificial Intelligence
  
- **UD**
  
  Uber DOG, see Uber DOGs.
  
- **OV**
  
  Owner View, see Owner View.
  
- **DC**
  
  Distributed Class, see Distributed Class Definition.
  
- **SR**
  
  Server Repository, see Server Repositories.
  
- **CR**
  
  Client Repository, see Client Repositories.

If you wish to learn more, you can also visit these resources available online:

- [October 2003: Building a MMOG for the Million - Disney's Toontown Online](https://dl.acm.org/doi/10.1145/950566.950589)
- [Apr 16, 2008: The DistributedObject System, client side](https://www.youtube.com/watch?v=JsgCFVpXQtQ)
- [Apr 23, 2008: DistributedObjects and the OTP server](https://www.youtube.com/watch?v=r_ZP9SInPcs)
- [Apr 30, 2008: OTP Server Internals](https://www.youtube.com/watch?v=SzybRdxjYoA)
- [October 2010: (GDC Online) MMO 101 - Building Disney's Server System](https://www.gdcvault.com/play/1013776/MMO-101-Building-Disney-s)
- [(PDF Slideshow) MMO 101 - Building Disney's Server System](https://ubm-twvideo01.s3.amazonaws.com/o1/vault/gdconline10/slides/11516-MMO_101_Building_Disneys_Sever.pdf)


