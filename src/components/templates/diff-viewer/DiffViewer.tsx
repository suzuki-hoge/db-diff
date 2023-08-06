import { type FC, Fragment, useState } from 'react'

import styles from './DiffViewer.module.scss'
import { type TableDiff } from '../../../types'
import { DiffContent } from '../../organisms/diff-content/DiffContent'
import { IconVisible } from '../../atoms/icon-visible/IconVisible'
import { Header } from '../../molecules/header/Header'
import { IconBack } from '../../atoms/icon-back/IconBack'
import { ModalWrapper } from '../../molecules/modal-wrapper/ModalWrapper'
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
              {props.tableDiffs.length !== 0 && (
                <IconSearch
                  variant={'large'}
                  onClick={() => {
                    setIsModalOpen(true)
                  }}
                />
              )}
              {props.tableDiffs.length !== 0 && (
                <IconHelp
                  variant={'large'}
                  onClick={() => {
                    setIsTouring(true)
                  }}
                />
              )}
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
          {props.tableDiffs.length === 0 && (
            <div className={styles.empty}>
              <p>2 つのスナップショットに差分がありません</p>
              <p>新たなスナップショットを作成して、別の差分表示を見てみましょう</p>
            </div>
          )}
        </div>
        <ModalWrapper isOpen={isModalOpen} setIsOpen={setIsModalOpen}>
          {props.tableDiffs
            .map((diff) => diff.tableName)
            .map((tableName, i) => (
              <div key={i} className={styles.item}>
                {ignoreTableNames.includes(tableName) ? (
                  <span className={styles.gray}>{tableName}</span>
                ) : (
                  <span
                    className={styles.anchor}
                    onClick={() => {
                      const target = document.getElementById(tableName)

                      if (target !== null) {
                        target.scrollIntoView({ behavior: 'smooth' })
                      }

                      setIsModalOpen(false)
                    }}
                  >
                    {tableName}
                  </span>
                )}
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
        </ModalWrapper>
      </div>
      <TourWrapper
        steps={steps}
        isTouring={isTouring}
        onClose={() => {
          setIsTouring(false)
        }}
      />
    </>
  )
}

const steps: ReactourStep[] = [
  {
    selector: '.icon_search',
    content: '表示するテーブルをフィルタリングできます',
  },
  {
    content: '主キーごとに変更差分が表示されます',
  },
  {
    content: (
      <>
        <span>
          主キーとみなせるカラム構成については <a href={'todo'}>GitHub のドキュメント</a> を確認してください
        </span>
      </>
    ),
  },
  {
    content: '主キーのないテーブルはスナップショット作成時に除外されています',
  },
]
