import { type FC, useState } from 'react'
import styles from './ProjectList.module.scss'
import { type Project } from '../../../types'
import { ColorTagCard } from '../../molecules/color-tag-card/ColorTagCard'
import { Header } from '../../molecules/header/Header'
import { IconPlus } from '../../atoms/icon-plus/IconPlus'
import { IconGear } from '../../atoms/icon-gear/IconGear'
import { IconEdit } from '../../atoms/icon-edit/IconEdit'
import { IconDelete } from '../../atoms/icon-delete/IconDelete'
import { useNavigate } from 'react-router-dom'
import { IconHelp } from '../../atoms/icon-help/IconHelp'
import { type ReactourStep } from 'reactour'
import { TourWrapper } from '../../atoms/tour-wrapper/TourWrapper'

interface Props {
  projects: Project[]
  select: (projectId: string) => void
  remove: (id: string) => void
}

export const ProjectList: FC<Props> = (props) => {
  const [isTouring, setIsTouring] = useState(false)
  const [isSetting, setIsSetting] = useState(false)

  const navigate = useNavigate()

  return (
    <>
      <div className={styles.template}>
        <Header
          globals={<></>}
          locals={
            <>
              <IconPlus
                variant={'large'}
                onClick={() => {
                  navigate('/project/create')
                }}
              />
              <IconGear
                variant={'large'}
                onClick={() => {
                  setIsSetting(!isSetting)
                }}
              />
              <IconHelp
                variant={'large'}
                onClick={() => {
                  setIsSetting(true)
                  setIsTouring(true)
                }}
              />
            </>
          }
        />
        <div className={styles.component}>
          <div className={styles.content}>
            {props.projects.map((project, i) => (
              <div key={i} className={styles.item}>
                <ColorTagCard
                  label={project.name}
                  color={project.color}
                  onClick={() => {
                    props.select(project.projectId)
                  }}
                />
                {isSetting && (
                  <div className={styles.icons}>
                    <IconEdit
                      variant={'medium'}
                      onClick={() => {
                        navigate('/project/update', { state: project })
                      }}
                    />
                    <IconDelete
                      variant={'medium'}
                      onClick={() => {
                        props.remove(project.projectId)
                      }}
                    />
                  </div>
                )}
              </div>
            ))}
          </div>
          {props.projects.length === 0 && (
            <div className={styles.empty}>
              <p>接続設定がありません</p>
              <p>
                チュートリアルツアー{' '}
                <IconHelp
                  variant={'small'}
                  onClick={() => {
                    setIsTouring(true)
                  }}
                />{' '}
                を見て接続設定を作成しましょう
              </p>
            </div>
          )}
        </div>
      </div>
      <TourWrapper
        steps={steps(props.projects.length === 0)}
        isTouring={isTouring}
        onClose={() => {
          setIsTouring(false)
          setIsSetting(false)
        }}
      />
    </>
  )
}

const steps: (isEmpty: boolean) => ReactourStep[] = (isEmpty) => {
  return isEmpty
    ? [
        {
          selector: '.icon_plus',
          content: 'まずは接続設定を作成しましょう',
        },
        {
          content: '接続設定を作成したら選択して DB に接続しましょう',
        },
        {
          selector: '.icon_gear',
          content: '作成した接続設定を編集することもできます',
        },
      ]
    : [
        {
          selector: '.color_tag_card',
          content: '接続設定を選択して DB に接続しましょう',
        },
        {
          selector: '.icon_plus',
          content: '新たな接続設定を作成しましょう',
        },
        {
          selector: '.icon_gear',
          content: '作成した接続設定を編集することができます',
        },
        {
          selector: '.icon_edit',
          content: '接続設定を編集できます',
        },
        {
          selector: '.icon_delete',
          content: '接続設定を削除できます',
        },
      ]
}
