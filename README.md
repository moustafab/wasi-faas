# wasi-faas

A lightweight FaaS executor that uses WASI to run WebAssembly functions as a service.

## Architecture

There are 2 components to wasi-faas, the worker and the control-plane. The worker is responsible for executing the WebAssembly functions and the control-plane is responsible for assigning the worker and eventually the function uploading, entry management etc, as well as being the API gateway for incoming end-user requests.

The control-plane has a few responsibilities, but mostly stubbed out for now:
- [X] Central registration point for all workers
- [X] Accept incoming end-user requests and sending them along to the appropriate worker
- [ ] Identifying workers which are not responding and reassigning their functions to other workers and marking them as unhealthy
- [ ] Managing the function source code and uploading to some object storage and storing the metadata about the function and it's listener endpoint

The worker has a relatively straightforward job:
- [X] Register with the control-plane with it's listener address
- [ ] Identify the function(s) it is responsible for and downloading the source code and instantiating the WebAssembly module
- [X] Listen for incoming requests from the control-plane for various handlers
- [X] Execute the WebAssembly function with the provided input and return the output


