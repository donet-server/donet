<img src="../logo/donet_banner.png" align="right" width="40%"/>

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
the communication, or the 'contract', the DoNet cluster which handles communication between 
clients, ClientRepositories which interact and manage the Distributed Objects, 
and the Distributed Objects themselves.

The architecture of a DoNet server cluster is made up of 6 different types of services:

### **[CA] - Client Agent**

  The Client Agent service manages connections with **anonymous clients** that are connecting 
  from outside of the internal server network. Clients do not directly communicate with the 
  DoNet cluster. Instead, the Client Agent relays client messages over to the network. This 
  component provides two of the main features in a DoNet server cluster, which is **security** 
  and **network culling**. It reads the DC file(s) given to it and approves all messages from 
  clients that conform to the communication 'contract' defined in the DC file. It also checks 
  for other details, such as the clients' ownership over Distributed Objects and the visibility 
  (or location) of Distributed Objects. The Client Agent acts as the border between the untrusted 
  clients and the internal server network's 'safe zone'.
  
### **[MD] - Message Director**
  
  The Message Director listens for messages from other services in a DoNet server cluster, 
  and **routes** them to other services based on the recipients in the message headers. A 
  message is a blob of binary data sent over the network, with a maximum size of approximately 
  **64 kilobytes**. The routing is performed by means of routing identifiers called **channels**, 
  where a message contains any number of destination channels, and most messages include a source, 
  or sender channel. Each service tells the Message Director which channels it would like to 
  **subscribe** to, and receives messages sent to its subscribed channels.
  
### **[SS] - State Server**
  
  The State Server service is responsible of coordinating the short-term existance of Distributed 
  Objects and their **states**. This component provides one of the main features in a DoNet server 
  cluster, which is **short-term persistance**. All Distributed Objects in a State Server exist 
  in memory and are part of a graph hierarchy called the **visibility tree**. The State Server has 
  data stored for each Distributed Object such as the class of the object, what its Distributed 
  Object ID (DoId) is, and where it is located in the visibility tree. Other services in a DoNet 
  cluster may communicate with the State Server through a Message Director to **manipulate** and 
  **query** Distributed Objects in the State Server's visibility tree.

### **[DB] - Database Server**
  
  The Database Server service is responsible for the long-term persistence of Distributed Object 
  **fields** that are marked in the DC file with a **"db" keyword**, which tells the Database 
  Server to store them on disk. It stores these fields in a **SQL database**, and can **update or 
  query** the Distributed Object's field's value.
  
### **[DBSS] - Database State Server**
  
  The Database State Server (DBSS for short) is a kind of **hybrid** service of a State 
  Server and a Database Server. This component is allows for other services in the cluster 
  to manipulate Distributed Object fields that are **currently not loaded on a State Server**. 
  The DBSS can also be configured to **listen to a range of DoId's** which it manages. If 
  it sees a location update for an object in its range, it will query the object from the 
  database and **convert it into a State Server object** in memory. For example, this is 
  useful if you have an avatar object that is currently offline and stored on the database. 
  If you would like to award a prize to the avatar while they're offline, the DBSS allows 
  you to query and manipulate the object even though it is not currently needed in memory 
  as the avatar is not actively 'present' in the visibility tree.
  
### **[EL] - Event Logger**
  
  The Event Logger listens to the Message Director for log messages that it should
  write to the disk. These log messages can be sent from AI processes, which are sent to
  a Message Director instance, which then routes it to the Event Logger. The Event Logger 
  is a useful tool for providing instrumentation to your server cluster and allows the 
  developer to analyze data in the game, depending on what the developer chooses to log.

<br>

DoNet can be configured to serve as all these roles under one daemon, which is 
handy for development on your local machine. For a production environment, many instances
of DoNet can be running on different machines and configured to serve as one service each. 
This configuration would be in a **.toml file** that the DoNet daemon would read on startup.

<br>

## Fundamental Terms & Concepts

There are many acronyms that you will find as you read the documentation. Please review the 
list below to learn and understand the different concepts and terms used in DoNet.

- **DO**

  Distributed Object. Represents an object present in a State Server's visibility tree.
  
- **DOG**
  
  Distributed Object Global. Similar to a Distributed Object, but is **omnipresent**
  in the Distributed Object visibility tree. This means that it is known globally and
  always remains accessible by all participants, such as Clients and AI processes.
  DOGs are **useful for authentication**, as anonymous (or non-authenticated) clients
  can interact with a Distributed Object Global object, as its not part of the visibility
  tree and it's DoId is a constant that is **globally known** by all clients.
  
- **DoId**
  
  Distributed Object Identifier. This is a **32-bit long identifier** that is generated
  at runtime to identify a Distributed Object that exists in the State Server.

- **DC**
  
  Distributed Class. This is a class definition that the developer defines in the DC file.
  Distributed Objects are instantiated based on a Distributed Class in which it **inherits**
  it's properties, or **fields**, from.
  
- **AI**
  
  Artificial Intelligence. The name for this is arbitrary, as it is not in any way 
  related to the field of machine learning. An AI is a process on the server cluster's 
  internal network that acts as a client connected directly to a Message Director instance. 
  This means that all AI clients bypass the Client Agent, as they are part of the 'safe zone.' 
  AI processes have **authority over Distributed Objects** and host the game/application's logic.
  
- **UD**
  
  Uber DOG. This is similar to an AI process, but is dedicated to managing Distributed
  Object Global (DOGs) objects.

- **Views**

  Views are local implementations of a Distributed Class from different **perspectives**.
  A view is essentially a representation of a Distributed Object in the eyes of a client.
  Distributed Object instances on a client inherit from a Distributed Class **and**
  are usually, by convention, named with a suffix which describes the object's
  perspective from the client's point of view in the virtual world. Valid suffixes
  are: **"AI"** (Artificial Intelligence), **"UD"** (UberDOG), and **"OV"** (Owner View).

  **"AI"** views are the AI-side representation of a Distributed Object instance.
  
  **"UD"** views are used by UberDOG processes (similar to AI clients).
  
  **"OV"** views are used by clients, which have **ownership** over that Distributed Object instance.

  Objects seen by a client, but not owned by it, also have their client-side representation without a suffix.

  Each view implements the logic that is executed when a field of the Distributed Object is called.
  For example, a Distributed Class named 'DistributedAvatar' has AI and OV views. The AI view
  may have the privilege to add currency to the 'DistributedAvatar' object, while the OV view has
  the ability to set the username of the 'DistributedAvatar' object. **In a nutshell**, AI processes
  implement administrative logic for a Distributed Object while client processes may implement special
  logic over objects they have ownership of, or use shared logic for objects they do not own.

  The concept of views is very similar to the
  [Model-view-controller (MVC)](https://en.wikipedia.org/wiki/Model%E2%80%93view%E2%80%93controller)
  software design pattern.
  
- **CR**
  
  Client Repository. See [Panda3D's Client Repository](https://docs.panda3d.org/1.10/python/programming/networking/distributed/client-repositories).

- **AIR**

  AI Repository. See [Panda3D's AI Repository](https://docs.panda3d.org/1.10/python/programming/networking/distributed/ai-repositories).

<br>

If you wish to learn more about Panda3D's Distributed Networking, you can also visit these resources available online:

- [October 2003: Building a MMOG for the Million - Disney's Toontown Online](https://dl.acm.org/doi/10.1145/950566.950589)
- [Apr 16, 2008: The DistributedObject System, client side](https://www.youtube.com/watch?v=JsgCFVpXQtQ)
- [Apr 23, 2008: DistributedObjects and the OTP server](https://www.youtube.com/watch?v=r_ZP9SInPcs)
- [Apr 30, 2008: OTP Server Internals](https://www.youtube.com/watch?v=SzybRdxjYoA)
- [October 2010: (GDC Online) MMO 101 - Building Disney's Server System](https://www.gdcvault.com/play/1013776/MMO-101-Building-Disney-s)
- [(PDF Slideshow) MMO 101 - Building Disney's Server System](https://ubm-twvideo01.s3.amazonaws.com/o1/vault/gdconline10/slides/11516-MMO_101_Building_Disneys_Sever.pdf)

