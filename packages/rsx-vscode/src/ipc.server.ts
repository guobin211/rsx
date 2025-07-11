import net from 'node:net'

const server = net.createServer((socket) => {
    socket.setEncoding('utf8')
    socket.setKeepAlive(true)
    socket.setTimeout(60 * 1000)
    let index = 0
    socket.on('data', (data) => {
        console.log('server received data: ')
        console.log(data.toString('utf-8'))
        index++
        socket.write(Buffer.from(`received ${index} messages`, 'utf8'))
    })
    socket.on('end', () => {
        console.log('socket client end')
    })
    socket.on('close', () => {
        console.log('socket client close')
    })
})

server.on('close', () => {
    console.log('server closed')
})

server.listen(8888, () => {
    console.log('server listening on 8888')
})
