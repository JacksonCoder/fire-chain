# FireChain
An ultra-fast protocol and system for semi-centralized record databases using blockchain technology.

## What is semi-centralization?
Usually, when we think of a blockchain-oriented system, we think of a decentralized network of computers that make up this system.
Decentralization of data has the following advantages:
- Total transparency of data
- No centralized authority
- Cryptographically verifiable by any entity

However, it also has a fair share of issues:
- All information must be fully transparent and shared, which may be undesirable for some use cases
- All nodes must keep a full copy of the ledger
- For data sharing networks, legal issues may arise if certain information is shared. For example,
if a blockchain network existed to contain information on student grades of certain schools, so that data could be 
aggregated for certain reasons, it would be a huge hassle legally to make sure all holders of this information consent to this
data being shared.
- In some cases, data aggregators are looking for a large, joined record from several sources, which, using a traditional 
decentralized structure, can be difficult to compile from many partial records.

A semi-centralization network solves those issues via it's design structure:
- Instead of hundreds of equal nodes, there are a handful of so called 'linker nodes', that act as a centralized
distributor of information.
- Each linker communicates with many 'partial' nodes of the network, each of which stores a partial record on it's database or other
storage method. This node is controlled by a lightweight FireChain 'interface daemon' which will provide information and read
from the database when it is passed authorized credentials.
- When someone wants to fetch a complete record, they create a 'Transaction', which is just some signed data telling the linker
node the hash or unique identifier of the whole record it wants to collect.
- The linker recieves the information, figures out the nodes it needs to communicate with, and then broadcasts the information
to the partial nodes. The nodes each relay their piece of the data, and then the linker puts it all together.
- The linker then returns the requested information

An example query from a client would look like this:
```
FETCH AS <client public key> : {
  RECORD: 4f3b125fd2a {
    grades_english => g_e,
    grades_math => g_m
  }
}
```

And an example response from the linker would look like this:
```json
{
  "success" : "true",
  "msg" : "No error occured",
  "data" : {
    "g_e" : "4.0",
    "g_m" : "3.8"
  }
}
```
