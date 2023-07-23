import type { Meta, StoryObj } from '@storybook/react'

import { Header } from './Header'
import { IconBack } from '../../atoms/icon-back/IconBack'
import { IconGear } from '../../atoms/icon-gear/IconGear'
import { IconDelete } from '../../atoms/icon-delete/IconDelete'
import { IconEdit } from '../../atoms/icon-edit/IconEdit'
import { IconSave } from '../../atoms/icon-save/IconSave'
import { IconVisible } from '../../atoms/icon-visible/IconVisible'
import { IconPlus } from '../../atoms/icon-plus/IconPlus'
import { IconClose } from '../../atoms/icon-close/IconClose'
import { IconSearch } from '../../atoms/icon-search/IconSearch'
import { IconHelp } from '../../atoms/icon-help/IconHelp'

const meta = {
  title: 'Molecules/Header',
  component: Header,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof Header>

export default meta
type Story = StoryObj<typeof meta>

export const LargeIcons: Story = {
  args: {
    globals: <IconBack variant={'large'} onClick={() => {}} />,
    locals: (
      <>
        <IconGear variant={'large'} onClick={() => {}} />
        <IconPlus variant={'large'} onClick={() => {}} />
        <IconSearch variant={'large'} onClick={() => {}} />
        <IconSave variant={'large'} onClick={() => {}} />
        <IconEdit variant={'large'} onClick={() => {}} />
        <IconDelete variant={'large'} onClick={() => {}} />
        <IconVisible variant={'large'} visible={true} onClick={() => {}} />
        <IconClose variant={'large'} onClick={() => {}} />
        <IconHelp variant={'large'} onClick={() => {}} />
      </>
    ),
  },
}

export const MediumIcons: Story = {
  args: {
    globals: <IconBack variant={'medium'} onClick={() => {}} />,
    locals: (
      <>
        <IconGear variant={'medium'} onClick={() => {}} />
        <IconPlus variant={'medium'} onClick={() => {}} />
        <IconSearch variant={'medium'} onClick={() => {}} />
        <IconSave variant={'medium'} onClick={() => {}} />
        <IconEdit variant={'medium'} onClick={() => {}} />
        <IconDelete variant={'medium'} onClick={() => {}} />
        <IconVisible variant={'medium'} visible={true} onClick={() => {}} />
        <IconClose variant={'medium'} onClick={() => {}} />
        <IconHelp variant={'medium'} onClick={() => {}} />
      </>
    ),
  },
}

export const SmallIcons: Story = {
  args: {
    globals: <IconBack variant={'small'} onClick={() => {}} />,
    locals: (
      <>
        <IconGear variant={'small'} onClick={() => {}} />
        <IconPlus variant={'small'} onClick={() => {}} />
        <IconSearch variant={'small'} onClick={() => {}} />
        <IconSave variant={'small'} onClick={() => {}} />
        <IconEdit variant={'small'} onClick={() => {}} />
        <IconDelete variant={'small'} onClick={() => {}} />
        <IconVisible variant={'small'} visible={true} onClick={() => {}} />
        <IconClose variant={'small'} onClick={() => {}} />
        <IconHelp variant={'small'} onClick={() => {}} />
      </>
    ),
  },
}

export const NoLocals: Story = {
  args: {
    globals: <IconBack variant={'large'} onClick={() => {}} />,
    locals: <></>,
  },
}
