type RecentsCardsProps = {
    title : string
    desc  : string
    atime : number
}

const timeAgo = (timestamp: number): string => {
  const currentTimeStampSeconds = Math.floor(Date.now()/1000)
  const diffInSeconds = Math.floor((currentTimeStampSeconds - timestamp))
  if (diffInSeconds < 60) return `${diffInSeconds}s ago`
  const diffInMinutes = Math.floor(diffInSeconds / 60)
  if (diffInMinutes < 60) return `${diffInMinutes}m ago`
  const diffInHours = Math.floor(diffInMinutes / 60)
  if (diffInHours < 24) return `${diffInHours}h ago`
  const diffInDays = Math.floor(diffInHours / 24)
  if (diffInDays < 30) return `${diffInDays}d ago`
  const diffInMonths = Math.floor(diffInDays / 30)
  if (diffInMonths < 12) return `${diffInMonths}mo ago`
  const diffInYears = Math.floor(diffInMonths / 12)
  return `${diffInYears}y ago`
}

const RecentsCard = ({title, desc, atime} : RecentsCardsProps) => (
  <div className='w-[344px] h-[77px] bg-white rounded-md border border-[#979797] grid grid-cols-[3fr_1fr] my-1'>
    <div className="p-2">
      <p className='font-extrabold text-sm line-clamp-1 break-all overflow-ellipsis'>{title}</p>
      <p className='font-light text-sm line-clamp-2 break-all overflow-ellipsis'>{desc}</p>
    </div>
    <div className='flex items-center justify-center'>
      <p className='text-sm font-light'>{timeAgo(atime)}</p>
    </div>
  </div>
)

export default RecentsCard
