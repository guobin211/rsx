import net from 'node:net'

const client = net.connect('8888')

client.on('connect', () => {
    console.log('client connected')
    client.write('client is connect')
})
client.on('ready', () => {
    console.log('client ready')
    client.write('client is ready')
})
client.on('data', (data) => {
    const msg = data.toString('utf8')
    console.log('client received', msg)
})
client.on('close', () => {
    console.log('client disconnected')
})
client.on('error', (err) => {
    console.error('client error', err)
})
