import { type FC, Fragment, useState } from 'react'

import styles from './DiffViewer.module.scss'
import { type TableDiff } from '../../../types'
import { DiffContent } from '../../organisms/diff-content/DiffContent'
import { IconVisible } from '../../atoms/icon-visible/IconVisible'
import { Header } from '../../molecules/header/Header'
import { IconBack } from '../../atoms/icon-back/IconBack'
import { ModalWindow } from '../../molecules/ModalWindow/ModalWindow'
import { IconSearch } from '../../atoms/icon-search/IconSearch'
import { useNavigate } from 'react-router-dom'

interface Props {
  tableDiffs: TableDiff[]
  ignoreTableNames: string[]
}

export const DiffViewer: FC<Props> = (props) => {
  const [ignoreTableNames, setIgnoreTableNames] = useState(props.ignoreTableNames)

  const [isModalOpen, setIsModalOpen] = useState(false)

  const navigate = useNavigate()

  return (
    <div className={styles.template}>
      <Header
        globals={
          <IconBack
            variant={'large'}
            onClick={() => {
              navigate('/snapshot-summary/list')
            }}
          />
        }
        locals={
          <IconSearch
            variant={'large'}
            onClick={() => {
              setIsModalOpen(true)
            }}
          />
        }
      />
      <div className={styles.component}>
        {props.tableDiffs.map((tableDiff) =>
          !ignoreTableNames.includes(tableDiff.tableName) ? (
            <DiffContent key={tableDiff.tableName} tableDiff={tableDiff} />
          ) : (
            <Fragment key={tableDiff.tableName}></Fragment>
          )
        )}
      </div>
      <ModalWindow isOpen={isModalOpen} setIsOpen={setIsModalOpen}>
        {props.tableDiffs
          .map((diff) => diff.tableName)
          .map((tableName, i) => (
            <div key={i} className={styles.item}>
              <span>{tableName}</span>
              <IconVisible
                variant={'medium'}
                visible={!ignoreTableNames.includes(tableName)}
                onClick={() => {
                  if (!ignoreTableNames.includes(tableName)) {
                    setIgnoreTableNames(ignoreTableNames.concat([tableName]))
                  } else {
                    setIgnoreTableNames(ignoreTableNames.filter((ignoreTableName) => ignoreTableName !== tableName))
                  }
                }}
              />
            </div>
          ))}
      </ModalWindow>
    </div>
  )
}
