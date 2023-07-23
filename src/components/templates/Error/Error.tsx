import { type FC } from 'react'
import styles from './Error.module.scss'
import { IconBack } from '../../atoms/icon-back/IconBack'
import { Header } from '../../molecules/header/Header'
import { useNavigate } from 'react-router-dom'

interface Props {
  message: string
}

export const Error: FC<Props> = (props) => {
  const navigate = useNavigate()

  return (
    <div className={styles.template}>
      <Header
        globals={
          <IconBack
            variant={'large'}
            onClick={() => {
              navigate('/project/list')
            }}
          />
        }
        locals={<></>}
      />
      <h2>なんらかのエラーが発生しました</h2>
      <p>DB を初期化するなり接続を確認するなり ( Todo )</p>
      <pre>{props.message}</pre>
    </div>
  )
}
