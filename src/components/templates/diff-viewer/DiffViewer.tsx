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
import { TourWrapper } from '../../atoms/tour-wrapper/TourWrapper'
import { type ReactourStep } from 'reactour'
import { IconHelp } from '../../atoms/icon-help/IconHelp'

interface Props {
  tableDiffs: TableDiff[]
}

export const DiffViewer: FC<Props> = (props) => {
  const [ignoreTableNames, setIgnoreTableNames] = useState<string[]>([])

  const [isTouring, setIsTouring] = useState(false)
  const [isModalOpen, setIsModalOpen] = useState(false)

  const navigate = useNavigate()

  return (
    <>
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
            <>
              <IconSearch
                variant={'large'}
                onClick={() => {
                  setIsModalOpen(true)
                }}
              />
              <IconHelp
                variant={'large'}
                onClick={() => {
                  setIsTouring(true)
                }}
              />
            </>
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
      <TourWrapper
        steps={steps(props.tableDiffs.length === 0)}
        isTouring={isTouring}
        onClose={() => {
          setIsTouring(false)
        }}
      />
    </>
  )
}

const steps: (isEmpty: boolean) => ReactourStep[] = (isEmpty) => {
  return isEmpty
    ? [
        {
          content: (
            <>
              <span>2 つのスナップショットに差分がありません</span>
              <br />
              <br />
              <span>新たなスナップショットを作成して、別の差分表示をみてみましょう</span>
            </>
          ),
        },
      ]
    : [
        {
          selector: '.icon_search',
          content: '表示するテーブルをフィルタリングできます',
        },
        {
          content: '主キーごとに変更差分が表示されます',
        },
      ]
}
