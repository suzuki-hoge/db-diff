import { type FC } from 'react'
import { Error } from '../components/templates/Error/Error'
import { useLocation } from 'react-router-dom'

export const ErrorPage: FC = () => {
  const location = useLocation()
  const { message } = location.state as { message: string }

  return <Error message={message} />
}
