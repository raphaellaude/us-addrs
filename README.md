# us-addrs

US Addrs is a rust package for parsing unstructured United States address strings into address components.
It is a rust implementation of the awesome [usaddress](https://github.com/datamade/usaddress/tree/master) library.
Thank you to the folks at [datamade](https://datamade.us/) for releasing such a cool tool.

US Addrs is currently *68% (~3x) faster* than usaddress, though additional optimizations should be possible. Accuracy stats TK.
The goal of this implementation is to faciliate use cases requiring better performance, such as geocoding large batches of addresses.
