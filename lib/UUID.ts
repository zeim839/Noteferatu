// UUID generates a unique note primary key id.
const UUID = () : number => {
  const buffer = new Uint32Array(2)
  crypto.getRandomValues(buffer)
  return (buffer[0] & 0x001fffff) * 0x100000000 + buffer[1]
}

export default UUID
