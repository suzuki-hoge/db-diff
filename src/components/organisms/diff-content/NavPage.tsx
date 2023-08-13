import React, { type FC } from 'react'
import { IconPage } from '../../atoms/icon-page/IconPage'
import styles from './NavPage.module.scss'

interface Props {
  s: number
  perpage: number
  len: number
  setS: (s: number) => void
}

export const NavPage: FC<Props> = (props) => {
  const head = 0
  const last = Math.max(0, props.len - props.perpage)

  return (
    <div className={styles.component}>
      <IconPage
        variant={'small'}
        vector={'left'}
        end={true}
        enabled={props.s !== head}
        onClick={() => {
          props.setS(head)
        }}
      />
      <IconPage
        variant={'small'}
        vector={'left'}
        end={false}
        enabled={props.s !== head}
        onClick={() => {
          props.setS(Math.max(head, props.s - props.perpage))
        }}
      />
      <span>
        {(props.s + 1).toLocaleString()} ~ {Math.min(props.s + props.perpage, props.len).toLocaleString()} of {props.len.toLocaleString()}
      </span>
      <IconPage
        variant={'small'}
        vector={'right'}
        end={false}
        enabled={props.s !== last}
        onClick={() => {
          props.setS(Math.min(last, props.s + props.perpage))
        }}
      />
      <IconPage
        variant={'small'}
        vector={'right'}
        end={true}
        enabled={props.s !== last}
        onClick={() => {
          props.setS(last)
        }}
      />
    </div>
  )
}
