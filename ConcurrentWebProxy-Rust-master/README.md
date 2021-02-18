# CSCI640 Concurrent Web Proxy Rust

## Introduction
A Web proxy is a program that acts as a middleman between a Web browser and an end server. Instead of contacting the end server directly to get a Web page, the browser contacts the proxy, which forwards the request on to the end server. When the end server replies to the proxy, the proxy sends the reply on to the browser.

Proxies are used for many purposes. Sometimes proxies are used in firewalls, such that the proxy is the only way for a browser inside the firewall to contact an end server outside. The proxy may do translation on the page, for instance, to make it viewable on a Web-enabled cell phone. Proxies are also used as anonymizers. By stripping a request of all identifying information, a proxy can make the browser anonymous to the end server. Proxies can even be used to cache Web objects, by storing a copy of, say, an image when a request for it is first made, and then serving that image in response to future requests rather than going to the end server.

In this lab, you will write a concurrent Web proxy that logs requests. In the first part of the lab, you will write a simple sequential proxy that repeatedly waits for a request, forwards the request to the end server, and returns the result back to the browser, keeping a log of such requests in a disk file. This part will help you understand basics about network programming and the HTTP protocol.

In the second part of the lab, you will upgrade your proxy so that it uses threads to deal with multiple clients concurrently. This part will give you some experience with concurrency and synchronization, which are crucial computer systems concepts.

## Logistics

The only handin will be electronic. Any clarifications and revisions to the assignment will be posted on the Piazza discussion board.

## Hand Out Instructions

Start by cloning the project files to a (protected) directory in which you plan to do your work. The project has the following files:

* **main.rs**: This is your source code file, which includes one helper function to help parse the proxy URI strings.

* **Cargo.toml**: The Rust cargo file, it may be necessary to update this so will be allowing you to submit your entire proxylab-handout directory as a tar.gz. 


## Part I: Implementing a Sequential Web Proxy
In this part you will implement a sequential logging proxy. Your proxy should open a socket and listen for a connection request. When it receives a connection request, it should accept the connection, read the HTTP request, and parse it to determine the name of the end server. It should then open a connection to the end server, send it the request, receive the reply, and forward the reply to the browser if the request is not blocked.

Since your proxy is a middleman between client and end server, it will have elements of both. It will act as a server to the web browser, and as a client to the end server. Thus you will get experience with both client and server programming.

### Logging
Your proxy should keep track of all requests in a log file named *proxy.log*. Each log file entry should be a file of the form:

```bash
Date: browserIP URL size
```

where *browserIP* is the IP address of the browser, *URL* is the URL asked for, *size* is the size in bytes of the object that was returned. For instance:

```bash
Sun 27 Oct 2002 02:51:02 EST: 128.2.111.38 http://www.cs.cmu.edu/ 34314
```

Note that *size* is essentially the number of bytes received from the end server, from the time the connection is opened to the time it is closed. Only requests that are met by a response from an end server should be logged. We have provided the function *format_log_entry* in *proxy.c* to create a log entry in the required format.

### Port Numbers
You proxy should listen for its connection requests on the port number passed in on the command line:

```bash
unix> ./proxy 15213
```

You may use any port number *p*, where *1024<p<65536*, and where *p* is not currently being used by any other system or user services (including other students’ proxies). See */etc/services* for a list of the port numbers reserved by other system services.

## Part II: Dealing with multiple requests concurrently
Real proxies do not process requests sequentially. They deal with multiple requests concurrently. Once you have a working sequential logging proxy, you should alter it to handle multiple requests concurrently. The simplest approach is to create a new thread to deal with each new connection request that arrives (CSAPP 13.3.8).

With this approach, it is possible for multiple peer threads to access the log file concurrently. Thus, you will need to use a semaphore to synchronize access to the file such that only one peer thread can modify it at a time. If you do not synchronize the threads, the log file might be corrupted. For instance, one line in the file might begin in the middle of another.

### Evaluation
Your code will be evaluated on the following basis.

* Basic proxy functionality (40 points). Your sequential proxy should correctly accept connections, forward the requests to the end server, and pass the response back to the browser, making a log entry for each request. Your program should be able to proxy browser requests to the following Web sites and correctly log the requests:
	* http://www.yahoo.com
	* http://www.aol.com
	* http://www.nfl.com

* Handling concurrent requests (20 points).

	* Your proxy should be able to handle multiple concurrent connections. We will determine this using the following test: (1) Open a connection to your proxy using *telnet*, and then leave it open without typing in any data. (2) Use a Web browser (pointed at your proxy) to request content from some end server.
	* Furthermore, your proxy should be thread-safe, protecting all updates of the log file and protecting calls to any thread unsafe functions such as *gethostbyaddr*. We will determine this by inspection during grading.
* Evaluation Quiz (40 points). You will have to be able to explain how your implementation works and handles concurrent requests if you got that functionality working on your Final.

## Hints
* The best way to get going on your proxy is to start with a basic echo server and then gradually add functionality that turns the server into a proxy.
* Initially, you should debug your proxy using telnet as the client (CS:APP 12.5.3).
* Later, test your proxy with a real browser. Explore the browser settings until you find “proxies”, then enter the host and port where you’re running yours. With Netscape, choose Edit, then Preferences, then Advanced, then Proxies, then Manual Proxy Configuration. In Internet Explorer, choose Tools, then Options, then Connections, then LAN Settings. Check ’Use proxy server,’ and click Advanced. Just set your HTTP proxy, because that’s all your code is going to be able to handle.
* Since we want you to focus on network programming issues for this lab, we have provided you with one additional helper routines: *parse_uri*, which extracts the hostname, path, and port components from a URI.
* Be careful about memory leaks. When the processing for an HTTP request fails for any reason, the thread must close all open socket descriptors and free all memory resources before terminating.
* Since the log file is being written to by multiple threads, you must protect it with some threadsafe mechanism. Your code will be checked for completeness. 


## Handin Instructions

* Remove any extraneous print statements.
* Make sure that you have included your identifying information in *main.rs*.
* Submit your *proxylab-handout.tar.gz* to Turnin, that is a compressed tar of your entire proxylab-handout directory of files. 
